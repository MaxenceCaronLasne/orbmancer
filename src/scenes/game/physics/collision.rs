use crate::scenes::game::{ball, Ball, peg::Pegs};
use agb::fixnum::num;

use super::constants::{
    LEFT_WALL, RIGHT_WALL, WALL_BOUNCE_DAMPING, ZERO,
    PhysicsConfig
};
use super::grid::NeighborStrategy;

/// Handle ball collisions with screen walls
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

/// Handle ball collisions with pegs using spatial optimization
/// 
/// This function uses a spatial grid to efficiently find nearby pegs,
/// avoiding the O(n) search through all pegs. Only computes expensive
/// sqrt when a collision is confirmed.
pub fn handle_ball_peg_collisions<T: NeighborStrategy>(
    ball: &mut Ball,
    pegs: &mut Pegs,
    strategy: &T,
    config: &PhysicsConfig,
) {
    let ball_radius = num!(ball::RADIUS);
    let peg_radius = num!(crate::scenes::game::peg::RADIUS);
    let collision_distance = ball_radius + peg_radius;
    let collision_distance_squared = collision_distance * collision_distance;

    for peg_id in strategy.get_neighbors(ball.position) {
        if peg_id >= pegs.count || pegs.is_touched(peg_id) {
            continue;
        }

        let distance_vector = ball.position - pegs.positions[peg_id];
        let distance_squared = distance_vector.magnitude_squared();

        if distance_squared < collision_distance_squared && distance_squared > num!(ZERO) {
            pegs.touch(peg_id);
            
            // Only compute expensive sqrt when collision is confirmed
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