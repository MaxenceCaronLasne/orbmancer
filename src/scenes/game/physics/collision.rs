use crate::scenes::game::peg::PegIndex;
use crate::scenes::game::{Ball, ball, peg::Pegs};
use agb::fixnum::num;

use super::constants::{
    LEFT_WALL, PhysicsConfig, RIGHT_WALL, WALL_BOUNCE_DAMPING, ZERO,
};

pub fn handle_ball_wall_collisions(ball: &mut Ball) {
    let ball_radius = num!(ball::RADIUS);

    if ball.position.x < num!(LEFT_WALL) + ball_radius {
        ball.position.x = num!(LEFT_WALL) + ball_radius;
        ball.velocity.x = -ball.velocity.x * num!(WALL_BOUNCE_DAMPING);
    } else if ball.position.x > num!(RIGHT_WALL) - ball_radius {
        ball.position.x = num!(RIGHT_WALL) - ball_radius;
        ball.velocity.x = -ball.velocity.x * num!(WALL_BOUNCE_DAMPING);
    }
}

pub fn handle_ball_peg_collisions(
    ball: &mut Ball,
    pegs: &mut Pegs,
    physics_state: &mut super::state::PhysicsState,
) {
    let ball_radius = num!(ball::RADIUS);
    let peg_radius = num!(crate::scenes::game::peg::RADIUS);
    let collision_distance = ball_radius + peg_radius;
    let collision_distance_squared = collision_distance * collision_distance;

    crate::bench::start("GRID");
    let neighbor_count = physics_state.fill_neighbors(ball.position);
    crate::bench::stop("GRID");

    let neighbors = physics_state.neighbors(neighbor_count);
    crate::bench::start("COLLISION");
    check_collisions(
        ball,
        pegs,
        neighbors,
        collision_distance_squared,
        physics_state.config(),
    );
    crate::bench::stop("COLLISION");
}

#[inline]
fn check_collisions(
    ball: &mut Ball,
    pegs: &mut Pegs,
    neighbors: &[PegIndex],
    collision_distance_squared: crate::types::Fixed,
    config: &PhysicsConfig,
) {
    for &peg_id in neighbors {
        if peg_id >= pegs.count || pegs.is_touched(peg_id) {
            continue;
        }
        let distance_vector = ball.position - pegs.position(peg_id);
        crate::bench::start("DISTANCE_CALC");
        let distance_squared = distance_vector.magnitude_squared();
        crate::bench::stop("DISTANCE_CALC");
        if distance_squared < collision_distance_squared
            && distance_squared > num!(ZERO)
        {
            pegs.touch(peg_id);
            crate::bench::start("COLLISION_RESPONSE");
            let collision_distance = (collision_distance_squared).sqrt();
            let distance = distance_squared.sqrt();
            let normal = distance_vector / distance;
            let velocity_along_normal = ball.velocity.dot(normal);
            ball.velocity -= normal * (velocity_along_normal * num!(2.0));
            ball.velocity *= config.peg_bounce_damping;
            let overlap = collision_distance - distance;
            ball.position += normal * overlap;
            crate::bench::stop("COLLISION_RESPONSE");
        }
    }
}
