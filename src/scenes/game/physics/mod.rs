use crate::error::Error;
use crate::scenes::game::peg::MAX_PEGS;
use crate::scenes::game::{ball, peg};
use crate::scenes::game::{ball::Ball, peg::Pegs};
use crate::types::{Coordinate, Fixed, Force};
use agb::fixnum::{num, vec2};
use grid::{Grid, NeighborStrategy};

pub mod grid;

const GRAVITY_Y: f32 = 200.0;
const LEFT_WALL: f32 = 0.0;
const RIGHT_WALL: f32 = 160.0;
const SCREEN_BOTTOM: f32 = 180.0;
const WALL_BOUNCE_DAMPING: f32 = 0.9;
const PEG_BOUNCE_DAMPING: f32 = 0.9;
const ZERO: f32 = 0.0;

const INTERACTION_FORCE_STRENGTH: f32 = 200.0;
const MAX_INTERACTION_DISTANCE_SQUARED: f32 = 60.0 * 60.0;
const PEG_MOVEMENT_LEFT_BOUND: f32 = 10.0;
const PEG_MOVEMENT_RIGHT_BOUND: f32 = 150.0;
const PEG_MOVEMENT_TOP_BOUND: f32 = 20.0;
const PEG_MOVEMENT_BOTTOM_BOUND: f32 = 140.0;

pub struct PhysicsState<T: NeighborStrategy> {
    neighbor_strategy: T,
}

impl<T: NeighborStrategy> PhysicsState<T> {
    pub fn new(neighbor_strategy: T) -> Self {
        Self { neighbor_strategy }
    }
}

pub fn new(pegs: &Pegs) -> PhysicsState<Grid> {
    PhysicsState::new(Grid::new(pegs))
}

fn handle_ball_wall_collisions(ball: &mut Ball) {
    let ball_radius = num!(ball::RADIUS);

    if ball.position.x < num!(LEFT_WALL) + ball_radius {
        ball.position.x = num!(LEFT_WALL) + ball_radius;
        ball.velocity.x = -ball.velocity.x * num!(WALL_BOUNCE_DAMPING);
    } else if ball.position.x > num!(RIGHT_WALL) - ball_radius {
        ball.position.x = num!(RIGHT_WALL) - ball_radius;
        ball.velocity.x = -ball.velocity.x * num!(WALL_BOUNCE_DAMPING);
    }
}

fn handle_ball_peg_collisions<T: NeighborStrategy>(
    ball: &mut Ball,
    pegs: &mut Pegs,
    strategy: &T,
) {
    let ball_radius = num!(ball::RADIUS);
    let peg_radius = num!(peg::RADIUS);

    for peg_id in strategy.get_neighbors(ball.position) {
        if peg_id >= pegs.count || pegs.is_touched(peg_id) {
            continue;
        }

        let distance_vector = ball.position - pegs.positions[peg_id];
        let distance = distance_vector.magnitude();
        let collision_distance = ball_radius + peg_radius;

        if distance < collision_distance && distance > num!(ZERO) {
            pegs.touch(peg_id);
            let normal = distance_vector / distance;
            let velocity_along_normal = ball.velocity.dot(normal);

            ball.velocity -= normal * (velocity_along_normal * num!(2.0));
            ball.velocity *= num!(PEG_BOUNCE_DAMPING);

            let overlap = collision_distance - distance;
            ball.position += normal * overlap;
        }
    }
}

fn calculate_peg_interaction_force(
    distance_squared: Fixed,
    field_size_squared: Fixed,
    direction_vector: Coordinate,
) -> Force {
    if distance_squared < field_size_squared {
        -direction_vector * num!(INTERACTION_FORCE_STRENGTH) * num!(0.1)
    } else {
        direction_vector
            * num!(INTERACTION_FORCE_STRENGTH)
            * num!(0.01)
            * num!(1.0)
            / distance_squared
    }
}

#[inline(always)]
fn apply_peg_force_pair(
    pegs: &Pegs,
    force_buffer: &mut [Force; MAX_PEGS],
    id_a: usize,
    id_b: usize,
) {
    let distance_vector = pegs.positions[id_b] - pegs.positions[id_a];
    let distance_squared = distance_vector.magnitude_squared();

    if distance_squared > num!(MAX_INTERACTION_DISTANCE_SQUARED) {
        return;
    }

    let field_size_a_squared = pegs.force_radii[id_a] * pegs.force_radii[id_a];
    let field_size_b_squared = pegs.force_radii[id_b] * pegs.force_radii[id_b];

    let force_on_a = calculate_peg_interaction_force(
        distance_squared,
        field_size_a_squared,
        distance_vector,
    );
    let force_on_b = calculate_peg_interaction_force(
        distance_squared,
        field_size_b_squared,
        -distance_vector,
    );

    force_buffer[id_a] += force_on_a;
    force_buffer[id_b] += force_on_b;
}

pub fn update_ball_physics<T: NeighborStrategy>(
    ball: &mut Ball,
    pegs: &mut Pegs,
    delta_time: Fixed,
    state: &PhysicsState<T>,
) {
    let initial_position = ball.position;
    let initial_velocity = ball.velocity;

    ball.velocity =
        initial_velocity + vec2(num!(0), num!(GRAVITY_Y)) * delta_time;
    ball.position = initial_position + ball.velocity * delta_time;

    handle_ball_wall_collisions(ball);

    if ball.position.y > num!(SCREEN_BOTTOM) {
        ball.velocity = Force::new(num!(ZERO), num!(ZERO));
        return;
    }

    handle_ball_peg_collisions(ball, pegs, &state.neighbor_strategy);
}

pub fn update_peg_physics<T: NeighborStrategy>(
    pegs: &mut Pegs,
    delta_time: Fixed,
    state: &PhysicsState<T>,
) -> Result<(), Error> {
    if pegs.count < 2 {
        return Ok(());
    }

    let mut force_buffer = [vec2(num!(0.0), num!(0.0)); MAX_PEGS];

    for i in 0..pegs.count {
        if !pegs.present[i] {
            continue;
        }

        for neighbor_id in
            state.neighbor_strategy.get_neighbors(pegs.positions[i])
        {
            if neighbor_id <= i {
                continue;
            }
            apply_peg_force_pair(pegs, &mut force_buffer, i, neighbor_id);
        }
    }

    #[allow(clippy::needless_range_loop)]
    for i in 0..pegs.count {
        if !pegs.present[i] {
            continue;
        }

        let velocity_change = force_buffer[i] * delta_time;
        pegs.positions[i] += velocity_change;

        pegs.positions[i].x = pegs.positions[i].x.clamp(
            num!(PEG_MOVEMENT_LEFT_BOUND),
            num!(PEG_MOVEMENT_RIGHT_BOUND),
        );
        pegs.positions[i].y = pegs.positions[i].y.clamp(
            num!(PEG_MOVEMENT_TOP_BOUND),
            num!(PEG_MOVEMENT_BOTTOM_BOUND),
        );
    }

    Ok(())
}
