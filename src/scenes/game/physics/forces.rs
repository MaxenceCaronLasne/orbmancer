use crate::scenes::game::peg::{MAX_PEGS, PegIndex, Pegs};
use crate::types::{Coordinate, Fixed, Force};
use agb::fixnum::{num, vec2};

use super::constants::{
    PEG_MOVEMENT_BOTTOM_BOUND, PEG_MOVEMENT_LEFT_BOUND,
    PEG_MOVEMENT_RIGHT_BOUND, PEG_MOVEMENT_TOP_BOUND, PhysicsConfig,
};
use super::grid::NeighborStrategy;

#[inline]
fn calculate_peg_interaction_force(
    distance_squared: Fixed,
    field_size_squared: Fixed,
    direction_vector: Coordinate,
    config: &PhysicsConfig,
) -> Force {
    if distance_squared < field_size_squared {
        -direction_vector * config.interaction_force_strength * num!(0.1)
    } else {
        direction_vector
            * config.interaction_force_strength
            * num!(0.01)
            * num!(1.0)
            / distance_squared
    }
}

#[inline(always)]
fn apply_peg_force_pair(
    pegs: &Pegs,
    force_buffer: &mut [Force; MAX_PEGS as usize],
    id_a: PegIndex,
    id_b: PegIndex,
    config: &PhysicsConfig,
) {
    let distance_vector = pegs.position(id_b) - pegs.position(id_a);
    let distance_squared = distance_vector.magnitude_squared();

    if distance_squared > config.max_interaction_distance_squared {
        return;
    }

    let field_size_a_squared = pegs.force_radius_squared(id_a);
    let field_size_b_squared = pegs.force_radius_squared(id_b);

    let force_on_a = calculate_peg_interaction_force(
        distance_squared,
        field_size_a_squared,
        distance_vector,
        config,
    );
    let force_on_b = calculate_peg_interaction_force(
        distance_squared,
        field_size_b_squared,
        -distance_vector,
        config,
    );

    force_buffer[id_a as usize] += force_on_a;
    force_buffer[id_b as usize] += force_on_b;
}

pub fn update_peg_physics<T: NeighborStrategy>(
    pegs: &mut Pegs,
    delta_time: Fixed,
    strategy: &T,
    config: &PhysicsConfig,
) {
    if pegs.count < 2 {
        return;
    }
    let mut force_buffer = [vec2(num!(0.0), num!(0.0)); MAX_PEGS as usize];
    for i in 0..pegs.count {
        if !pegs.is_present(i) {
            continue;
        }
        for neighbor_id in strategy.get_neighbors(pegs.position(i)) {
            if neighbor_id <= i {
                continue;
            }
            apply_peg_force_pair(
                pegs,
                &mut force_buffer,
                i,
                neighbor_id,
                config,
            );
        }
    }
    for i in 0..pegs.count {
        if !pegs.is_present(i) {
            continue;
        }
        let velocity_change = force_buffer[i as usize] * delta_time;
        let new_pos = pegs.position(i) + velocity_change;
        let x = new_pos.x.clamp(
            num!(PEG_MOVEMENT_LEFT_BOUND),
            num!(PEG_MOVEMENT_RIGHT_BOUND),
        );
        let y = new_pos.y.clamp(
            num!(PEG_MOVEMENT_TOP_BOUND),
            num!(PEG_MOVEMENT_BOTTOM_BOUND),
        );
        pegs.set_position(i, vec2(x, y));
    }
}
