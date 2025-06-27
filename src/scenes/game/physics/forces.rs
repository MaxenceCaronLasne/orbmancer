use crate::scenes::game::peg::{Pegs, MAX_PEGS};
use crate::types::{Coordinate, Fixed, Force};
use agb::fixnum::{num, vec2};

use super::constants::{
    PhysicsConfig, PEG_MOVEMENT_LEFT_BOUND, PEG_MOVEMENT_RIGHT_BOUND,
    PEG_MOVEMENT_TOP_BOUND, PEG_MOVEMENT_BOTTOM_BOUND
};
use super::grid::NeighborStrategy;

/// Calculate interaction force between two pegs based on distance and field sizes
/// 
/// Uses attraction/repulsion based on whether the distance is within the field size.
/// All calculations use squared distances to avoid expensive sqrt operations.
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

/// Apply interaction forces between two pegs
/// 
/// Uses pre-computed squared force radii to avoid repeated multiplications.
/// Only processes peg pairs within the maximum interaction distance.
#[inline(always)]
fn apply_peg_force_pair(
    pegs: &Pegs,
    force_buffer: &mut [Force; MAX_PEGS],
    id_a: usize,
    id_b: usize,
    config: &PhysicsConfig,
) {
    let distance_vector = pegs.positions[id_b] - pegs.positions[id_a];
    let distance_squared = distance_vector.magnitude_squared();

    if distance_squared > config.max_interaction_distance_squared {
        return;
    }

    let field_size_a_squared = pegs.force_radii_squared[id_a];
    let field_size_b_squared = pegs.force_radii_squared[id_b];

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

    force_buffer[id_a] += force_on_a;
    force_buffer[id_b] += force_on_b;
}

/// Update peg physics using neighbor strategy for spatial optimization
/// 
/// Calculates peg-peg interaction forces and applies them as position updates.
/// Uses spatial optimization to avoid O(n²) comparisons between all peg pairs.
/// 
/// # Performance Notes
/// - Uses magnitude_squared to avoid expensive sqrt operations
/// - Pre-computed force_radii_squared eliminates repeated multiplications
/// - Spatial grid reduces force calculations to nearby pegs only
pub fn update_peg_physics<T: NeighborStrategy>(
    pegs: &mut Pegs,
    delta_time: Fixed,
    strategy: &T,
    config: &PhysicsConfig,
) {
    update_peg_physics_with_timers(pegs, delta_time, strategy, config, None)
}

/// Update peg physics with optional timer for benchmarking
pub fn update_peg_physics_with_timers<T: NeighborStrategy>(
    pegs: &mut Pegs,
    delta_time: Fixed,
    strategy: &T,
    config: &PhysicsConfig,
    #[cfg(feature = "benchmark")]
    timers: Option<&agb::timer::Timers>,
    #[cfg(not(feature = "benchmark"))]
    _timers: Option<&agb::timer::Timers>,
) {
    if pegs.count < 2 {
        return;
    }

    let mut force_buffer = [vec2(num!(0.0), num!(0.0)); MAX_PEGS];

    // Calculate forces between nearby pegs
    #[cfg(feature = "benchmark")]
    let mut force_calculation = || {
        for i in 0..pegs.count {
            if !pegs.present[i] {
                continue;
            }

            for neighbor_id in strategy.get_neighbors(pegs.positions[i]) {
                if neighbor_id <= i {
                    continue; // Avoid duplicate calculations
                }
                apply_peg_force_pair(pegs, &mut force_buffer, i, neighbor_id, config);
            }
        }
    };

    #[cfg(not(feature = "benchmark"))]
    let mut force_calculation = || {
        for i in 0..pegs.count {
            if !pegs.present[i] {
                continue;
            }

            for neighbor_id in strategy.get_neighbors(pegs.positions[i]) {
                if neighbor_id <= i {
                    continue; // Avoid duplicate calculations
                }
                apply_peg_force_pair(pegs, &mut force_buffer, i, neighbor_id, config);
            }
        }
    };

    #[cfg(feature = "benchmark")]
    if let Some(timers) = timers {
        super::bench::PhysicsBench::measure_force_calculation(timers, force_calculation);
    } else {
        force_calculation();
    }

    #[cfg(not(feature = "benchmark"))]
    force_calculation();

    // Apply forces as velocity changes and clamp to boundaries
    #[cfg(feature = "benchmark")]
    let mut position_update = || {
        #[allow(clippy::needless_range_loop)]
        for i in 0..pegs.count {
            if !pegs.present[i] {
                continue;
            }

            let velocity_change = force_buffer[i] * delta_time;
            pegs.positions[i] += velocity_change;

            // Clamp peg positions to movement boundaries
            pegs.positions[i].x = pegs.positions[i].x.clamp(
                num!(PEG_MOVEMENT_LEFT_BOUND),
                num!(PEG_MOVEMENT_RIGHT_BOUND),
            );
            pegs.positions[i].y = pegs.positions[i].y.clamp(
                num!(PEG_MOVEMENT_TOP_BOUND),
                num!(PEG_MOVEMENT_BOTTOM_BOUND),
            );
        }
    };

    #[cfg(not(feature = "benchmark"))]
    let mut position_update = || {
        #[allow(clippy::needless_range_loop)]
        for i in 0..pegs.count {
            if !pegs.present[i] {
                continue;
            }

            let velocity_change = force_buffer[i] * delta_time;
            pegs.positions[i] += velocity_change;

            // Clamp peg positions to movement boundaries
            pegs.positions[i].x = pegs.positions[i].x.clamp(
                num!(PEG_MOVEMENT_LEFT_BOUND),
                num!(PEG_MOVEMENT_RIGHT_BOUND),
            );
            pegs.positions[i].y = pegs.positions[i].y.clamp(
                num!(PEG_MOVEMENT_TOP_BOUND),
                num!(PEG_MOVEMENT_BOTTOM_BOUND),
            );
        }
    };

    #[cfg(feature = "benchmark")]
    if let Some(timers) = timers {
        super::bench::PhysicsBench::measure_peg_update(timers, position_update);
    } else {
        position_update();
    }

    #[cfg(not(feature = "benchmark"))]
    position_update();
}