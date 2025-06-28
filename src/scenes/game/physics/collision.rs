use super::constants::{
    LEFT_WALL, PhysicsConfig, RIGHT_WALL, WALL_BOUNCE_DAMPING, ZERO,
};
use super::state::PhysicsState;
use crate::scenes::game::peg::PegIndex;
use crate::scenes::game::{Ball, ball, peg::Pegs};
use agb::fixnum::num;

const COLLISION_DISTANCE_F32: f32 =
    ball::RADIUS + crate::scenes::game::peg::RADIUS;
const COLLISION_DISTANCE_SQUARED_F32: f32 =
    COLLISION_DISTANCE_F32 * COLLISION_DISTANCE_F32;

const PEG_COLLISION_DISTANCE_F32: f32 = 2.0 * crate::scenes::game::peg::RADIUS;
const PEG_COLLISION_DISTANCE_SQUARED_F32: f32 =
    PEG_COLLISION_DISTANCE_F32 * PEG_COLLISION_DISTANCE_F32;

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
    physics_state: &mut PhysicsState,
) {
    let neighbor_count = physics_state.fill_neighbors(ball.position);

    let neighbors = physics_state.neighbors(neighbor_count);

    check_ball_collisions(
        ball,
        pegs,
        neighbors,
        num!(COLLISION_DISTANCE_SQUARED_F32),
        physics_state.config(),
    );
}

pub fn handle_peg_peg_collisions(
    pegs: &mut Pegs,
    physics_state: &mut PhysicsState,
) {
    for i in 0..pegs.count {
        let neighbor_count = physics_state.fill_neighbors(pegs.position(i));
        let neighbors = physics_state.neighbors(neighbor_count);

        check_peg_collisions(
            i,
            pegs,
            neighbors,
            num!(PEG_COLLISION_DISTANCE_SQUARED_F32),
            physics_state.config(),
        );
    }
}

struct CollisionInfo {
    distance: crate::types::Fixed,
    normal: crate::types::Coordinate,
    overlap: crate::types::Fixed,
}

#[inline]
fn calculate_collision_info(
    pos1: crate::types::Coordinate,
    pos2: crate::types::Coordinate,
    collision_distance_squared: crate::types::Fixed,
    collision_distance: crate::types::Fixed,
) -> Option<CollisionInfo> {
    let distance_vector = pos1 - pos2;
    let distance_squared = distance_vector.magnitude_squared();
    
    if distance_squared < collision_distance_squared && distance_squared > num!(ZERO) {
        let distance = distance_squared.sqrt();
        let inv_distance = num!(1.0) / distance;
        let normal = distance_vector * inv_distance;
        let overlap = collision_distance - distance;
        
        Some(CollisionInfo {
            distance,
            normal,
            overlap,
        })
    } else {
        None
    }
}

#[inline]
fn check_ball_collisions(
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
            let distance = distance_squared.sqrt();
            let inv_distance = num!(1.0) / distance;
            let normal = distance_vector * inv_distance;
            let velocity_along_normal = ball.velocity.dot(normal);
            ball.velocity -= normal * (velocity_along_normal * num!(2.0));
            ball.velocity *= config.peg_bounce_damping;
            let overlap = num!(COLLISION_DISTANCE_F32) - distance;
            ball.position += normal * overlap;
        }
    }
}

#[inline]
fn check_peg_collisions(
    index: PegIndex,
    pegs: &mut Pegs,
    neighbors: &[PegIndex],
    collision_distance_squared: crate::types::Fixed,
    config: &PhysicsConfig,
) {
    for &peg_id in neighbors {
        if peg_id >= pegs.count || peg_id == index {
            continue;
        }
        
        let distance_vector = pegs.position(index) - pegs.position(peg_id);
        let distance_squared = distance_vector.magnitude_squared();
        
        if distance_squared > num!(ZERO) && distance_squared < collision_distance_squared {
            let distance = distance_squared.sqrt();
            let normal = distance_vector / distance;
            
            let force_magnitude = config.peg_repulsion_strength / distance;
            let repulsion_force = normal * force_magnitude;
            
            let velocity1 = pegs.velocity(index);
            let velocity2 = pegs.velocity(peg_id);
            
            pegs.set_velocity(index, velocity1 + repulsion_force);
            pegs.set_velocity(peg_id, velocity2 - repulsion_force);
        }
    }
}
