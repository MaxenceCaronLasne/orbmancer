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
    update_peg_physics_with_timers(pegs, delta_time, strategy, config, None)
}

pub fn update_peg_physics_with_timers<T: NeighborStrategy>(
    pegs: &mut Pegs,
    delta_time: Fixed,
    strategy: &T,
    config: &PhysicsConfig,
    #[cfg(feature = "benchmark")] timers: Option<&agb::timer::Timers>,
    #[cfg(not(feature = "benchmark"))] _timers: Option<&agb::timer::Timers>,
) {
    if pegs.count < 2 {
        return;
    }

    let mut force_buffer = [vec2(num!(0.0), num!(0.0)); MAX_PEGS as usize];

    // Calculate forces between nearby pegs
    #[cfg(feature = "benchmark")]
    let mut force_calculation = || {
        for i in 0..pegs.count {
            if !pegs.is_present(i) {
                continue;
            }

            for neighbor_id in strategy.get_neighbors(pegs.position(i)) {
                if neighbor_id <= i {
                    continue; // Avoid duplicate calculations
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
                apply_peg_force_pair(
                    pegs,
                    &mut force_buffer,
                    i,
                    neighbor_id,
                    config,
                );
            }
        }
    };

    #[cfg(feature = "benchmark")]
    if let Some(timers) = timers {
        super::bench::PhysicsBench::measure_force_calculation(
            timers,
            force_calculation,
        );
    } else {
        force_calculation();
    }

    #[cfg(not(feature = "benchmark"))]
    force_calculation();

    #[cfg(feature = "benchmark")]
    let mut position_update = || {
        #[allow(clippy::needless_range_loop)]
        for i in 0..pegs.count {
            if !pegs.is_present(i) {
                continue;
            }

            let velocity_change = force_buffer[i as usize] * delta_time;
            pegs.set_position(i, pegs.position(i) + velocity_change);

            let x = pegs.position(i).x.clamp(
                num!(PEG_MOVEMENT_LEFT_BOUND),
                num!(PEG_MOVEMENT_RIGHT_BOUND),
            );
            let y = pegs.position(i).y.clamp(
                num!(PEG_MOVEMENT_TOP_BOUND),
                num!(PEG_MOVEMENT_BOTTOM_BOUND),
            );

            pegs.set_position(i, vec2(x, y));
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

/// Update peg physics using pre-allocated buffer for optimal performance
pub fn update_peg_physics_with_buffer<T: NeighborStrategy>(
    pegs: &mut Pegs,
    delta_time: Fixed,
    state: &mut super::state::PhysicsState<T>,
    config: &PhysicsConfig,
    #[cfg(feature = "benchmark")] timers: Option<&agb::timer::Timers>,
    #[cfg(not(feature = "benchmark"))] _timers: Option<&agb::timer::Timers>,
) {
    if pegs.count < 2 {
        return;
    }

    let mut force_buffer = [vec2(num!(0.0), num!(0.0)); MAX_PEGS];

    // Calculate forces between nearby pegs using pre-allocated buffer
    #[cfg(feature = "benchmark")]
    let mut force_calculation = || {
        for i in 0..pegs.count {
            if !pegs.is_present(i) {
                continue;
            }

            let neighbor_count = state.fill_neighbor_buffer(pegs.position(i));
            let neighbors = state.neighbor_slice(neighbor_count);

            for &neighbor_id in neighbors {
                if neighbor_id <= i {
                    continue; // Avoid duplicate calculations
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
    };

    #[cfg(not(feature = "benchmark"))]
    let mut force_calculation = || {
        for i in 0..pegs.count {
            if !pegs.present[i] {
                continue;
            }

            let neighbor_count = state.fill_neighbor_buffer(pegs.positions[i]);
            let neighbors = state.neighbor_slice(neighbor_count);

            for &neighbor_id in neighbors {
                if neighbor_id <= i {
                    continue; // Avoid duplicate calculations
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
    };

    #[cfg(feature = "benchmark")]
    if let Some(timers) = timers {
        super::bench::PhysicsBench::measure_force_calculation(
            timers,
            force_calculation,
        );
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
            if !pegs.is_present(i) {
                continue;
            }

            let velocity_change = force_buffer[i as usize] * delta_time;
            pegs.set_position(i, pegs.position(i) + velocity_change);

            let x = pegs.position(i).x.clamp(
                num!(PEG_MOVEMENT_LEFT_BOUND),
                num!(PEG_MOVEMENT_RIGHT_BOUND),
            );
            let y = pegs.position(i).y.clamp(
                num!(PEG_MOVEMENT_TOP_BOUND),
                num!(PEG_MOVEMENT_BOTTOM_BOUND),
            );

            pegs.set_position(i, vec2(x, y));
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

