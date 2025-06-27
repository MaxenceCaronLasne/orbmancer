use crate::scenes::game::peg::PegIndex;
use crate::scenes::game::{Ball, ball, peg::Pegs};
use agb::fixnum::num;

use super::constants::{
    LEFT_WALL, PhysicsConfig, RIGHT_WALL, WALL_BOUNCE_DAMPING, ZERO,
};
use super::grid::NeighborStrategy;

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

pub fn handle_ball_peg_collisions<T: NeighborStrategy>(
    ball: &mut Ball,
    pegs: &mut Pegs,
    strategy: &T,
    config: &PhysicsConfig,
) {
    handle_ball_peg_collisions_with_timers(ball, pegs, strategy, config, None)
}

/// Handle ball collisions with pegs with optional timer for benchmarking
pub fn handle_ball_peg_collisions_with_timers<T: NeighborStrategy>(
    ball: &mut Ball,
    pegs: &mut Pegs,
    strategy: &T,
    config: &PhysicsConfig,
    #[cfg(feature = "benchmark")] timers: Option<&agb::timer::Timers>,
    #[cfg(not(feature = "benchmark"))] _timers: Option<&agb::timer::Timers>,
) {
    let ball_radius = num!(ball::RADIUS);
    let peg_radius = num!(crate::scenes::game::peg::RADIUS);
    let collision_distance = ball_radius + peg_radius;
    let collision_distance_squared = collision_distance * collision_distance;

    #[cfg(feature = "benchmark")]
    if let Some(timers) = timers {
        super::bench::PhysicsBench::measure_grid_query(timers, || {
            let neighbors = strategy.get_neighbors(ball.position);
            super::bench::PhysicsBench::measure_collision(timers, || {
                perform_collision_checks_iter(
                    ball,
                    pegs,
                    neighbors,
                    collision_distance_squared,
                    config,
                )
            })
        })
    } else {
        let neighbors = strategy.get_neighbors(ball.position);
        perform_collision_checks_iter(
            ball,
            pegs,
            neighbors,
            collision_distance_squared,
            config,
        )
    };

    #[cfg(not(feature = "benchmark"))]
    {
        let neighbors = strategy.get_neighbors(ball.position);
        perform_collision_checks_iter(
            ball,
            pegs,
            neighbors,
            collision_distance_squared,
            config,
        )
    }
}

pub fn handle_ball_peg_collisions_with_buffer<T: NeighborStrategy>(
    ball: &mut Ball,
    pegs: &mut Pegs,
    physics_state: &mut super::state::PhysicsState<T>,
    #[cfg(feature = "benchmark")] timers: Option<&agb::timer::Timers>,
    #[cfg(not(feature = "benchmark"))] _timers: Option<&agb::timer::Timers>,
) {
    let ball_radius = num!(ball::RADIUS);
    let peg_radius = num!(crate::scenes::game::peg::RADIUS);
    let collision_distance = ball_radius + peg_radius;
    let collision_distance_squared = collision_distance * collision_distance;

    #[cfg(feature = "benchmark")]
    if let Some(timers) = timers {
        let neighbor_count =
            super::bench::PhysicsBench::measure_grid_query(timers, || {
                physics_state.fill_neighbor_buffer(ball.position)
            });
        let neighbors = physics_state.neighbor_slice(neighbor_count);
        super::bench::PhysicsBench::measure_collision(timers, || {
            perform_collision_checks_slice(
                ball,
                pegs,
                neighbors,
                collision_distance_squared,
                physics_state.config(),
            )
        })
    } else {
        let neighbor_count = physics_state.fill_neighbor_buffer(ball.position);
        let neighbors = physics_state.neighbor_slice(neighbor_count);
        perform_collision_checks_slice(
            ball,
            pegs,
            neighbors,
            collision_distance_squared,
            physics_state.config(),
        )
    };

    #[cfg(not(feature = "benchmark"))]
    {
        let neighbor_count = physics_state.fill_neighbor_buffer(ball.position);
        let neighbors = physics_state.neighbor_slice(neighbor_count);
        perform_collision_checks_slice(
            ball,
            pegs,
            neighbors,
            collision_distance_squared,
            physics_state.config(),
        )
    }
}

fn perform_collision_checks_iter(
    ball: &mut Ball,
    pegs: &mut Pegs,
    neighbors: impl Iterator<Item = PegIndex>,
    collision_distance_squared: crate::types::Fixed,
    config: &PhysicsConfig,
) {
    for peg_id in neighbors {
        if peg_id >= pegs.count || pegs.is_touched(peg_id) {
            continue;
        }

        let distance_vector = ball.position - pegs.position(peg_id);
        let distance_squared = distance_vector.magnitude_squared();

        if distance_squared < collision_distance_squared
            && distance_squared > num!(ZERO)
        {
            pegs.touch(peg_id);

            // Only compute expensive sqrt when collision is confirmed
            let collision_distance = (collision_distance_squared).sqrt();
            let distance = distance_squared.sqrt();
            let normal = distance_vector / distance;
            let velocity_along_normal = ball.velocity.dot(normal);

            ball.velocity -= normal * (velocity_along_normal * num!(2.0));
            ball.velocity *= config.peg_bounce_damping;

            let overlap = collision_distance - distance;
            ball.position += normal * overlap;
        }
    }
}

/// Perform collision detection and response using pre-allocated slice
#[inline]
fn perform_collision_checks_slice(
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
        let distance_squared = distance_vector.magnitude_squared();

        if distance_squared < collision_distance_squared
            && distance_squared > num!(ZERO)
        {
            pegs.touch(peg_id);

            // Only compute expensive sqrt when collision is confirmed
            let collision_distance = (collision_distance_squared).sqrt();
            let distance = distance_squared.sqrt();
            let normal = distance_vector / distance;
            let velocity_along_normal = ball.velocity.dot(normal);

            ball.velocity -= normal * (velocity_along_normal * num!(2.0));
            ball.velocity *= config.peg_bounce_damping;

            let overlap = collision_distance - distance;
            ball.position += normal * overlap;
        }
    }
}

