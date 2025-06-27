use crate::error::Error;
use crate::scenes::game::peg::PegIndex;
use crate::scenes::game::{Ball, peg::Pegs};
use crate::types::Fixed;
use agb::fixnum::{num, vec2};

use super::collision::{
    handle_ball_peg_collisions, handle_ball_peg_collisions_with_timers,
    handle_ball_wall_collisions,
};
use super::constants::{PhysicsConfig, SCREEN_BOTTOM, ZERO};
use super::forces::{update_peg_physics, update_peg_physics_with_timers};
use super::grid::{Grid, NeighborStrategy};

const MAX_NEIGHBORS: usize = 32;

pub struct PhysicsState<T: NeighborStrategy> {
    neighbor_strategy: T,
    config: PhysicsConfig,
    neighbor_buffer: [PegIndex; MAX_NEIGHBORS],
}

impl<T: NeighborStrategy> PhysicsState<T> {
    #[must_use]
    pub fn new(neighbor_strategy: T) -> Self {
        Self {
            neighbor_strategy,
            config: PhysicsConfig::default(),
            neighbor_buffer: [0; MAX_NEIGHBORS],
        }
    }

    #[must_use]
    pub fn with_config(neighbor_strategy: T, config: PhysicsConfig) -> Self {
        Self {
            neighbor_strategy,
            config,
            neighbor_buffer: [0; MAX_NEIGHBORS],
        }
    }

    #[must_use]
    pub fn config(&self) -> &PhysicsConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut PhysicsConfig {
        &mut self.config
    }

    #[inline]
    pub fn fill_neighbor_buffer(
        &mut self,
        position: crate::types::Coordinate,
    ) -> usize {
        let mut count = 0;
        for neighbor_id in self.neighbor_strategy.get_neighbors(position) {
            if count >= MAX_NEIGHBORS {
                break;
            }
            self.neighbor_buffer[count] = neighbor_id;
            count += 1;
        }
        count
    }

    #[inline]
    pub fn neighbor_slice(&self, count: usize) -> &[PegIndex] {
        &self.neighbor_buffer[..count.min(MAX_NEIGHBORS)]
    }
}

impl PhysicsState<Grid> {
    pub fn update_grid(&mut self, pegs: &Pegs) {
        self.neighbor_strategy.update(pegs);
    }
}

#[must_use]
pub fn new(pegs: &Pegs) -> PhysicsState<Grid> {
    PhysicsState::new(Grid::new(pegs))
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
        initial_velocity + vec2(num!(0), state.config.gravity_y) * delta_time;
    ball.position = initial_position + ball.velocity * delta_time;

    handle_ball_wall_collisions(ball);

    if ball.position.y > num!(SCREEN_BOTTOM) {
        ball.velocity = vec2(num!(ZERO), num!(ZERO));
        return;
    }

    handle_ball_peg_collisions(
        ball,
        pegs,
        &state.neighbor_strategy,
        &state.config,
    );
}

pub fn update_ball_physics_with_buffer<T: NeighborStrategy>(
    ball: &mut Ball,
    pegs: &mut Pegs,
    delta_time: Fixed,
    state: &mut PhysicsState<T>,
    #[cfg(feature = "benchmark")] timers: Option<&agb::timer::Timers>,
    #[cfg(not(feature = "benchmark"))] _timers: Option<&agb::timer::Timers>,
) {
    let initial_position = ball.position;
    let initial_velocity = ball.velocity;

    // Apply gravity and update position
    ball.velocity =
        initial_velocity + vec2(num!(0), state.config.gravity_y) * delta_time;
    ball.position = initial_position + ball.velocity * delta_time;

    handle_ball_wall_collisions(ball);

    // Reset ball if it falls below screen
    if ball.position.y > num!(SCREEN_BOTTOM) {
        ball.velocity = vec2(num!(ZERO), num!(ZERO));
        return;
    }

    #[cfg(feature = "benchmark")]
    super::collision::handle_ball_peg_collisions_with_buffer(
        ball, pegs, state, timers,
    );

    #[cfg(not(feature = "benchmark"))]
    super::collision::handle_ball_peg_collisions_with_buffer(
        ball, pegs, state, None,
    );
}

/// Update ball physics with benchmarking support
pub fn update_ball_physics_with_timers<T: NeighborStrategy>(
    ball: &mut Ball,
    pegs: &mut Pegs,
    delta_time: Fixed,
    state: &PhysicsState<T>,
    #[cfg(feature = "benchmark")] timers: Option<&agb::timer::Timers>,
    #[cfg(not(feature = "benchmark"))] _timers: Option<&agb::timer::Timers>,
) {
    let initial_position = ball.position;
    let initial_velocity = ball.velocity;

    // Apply gravity and update position
    ball.velocity =
        initial_velocity + vec2(num!(0), state.config.gravity_y) * delta_time;
    ball.position = initial_position + ball.velocity * delta_time;

    handle_ball_wall_collisions(ball);

    // Reset ball if it falls below screen
    if ball.position.y > num!(SCREEN_BOTTOM) {
        ball.velocity = vec2(num!(ZERO), num!(ZERO));
        return;
    }

    #[cfg(feature = "benchmark")]
    handle_ball_peg_collisions_with_timers(
        ball,
        pegs,
        &state.neighbor_strategy,
        &state.config,
        timers,
    );

    #[cfg(not(feature = "benchmark"))]
    handle_ball_peg_collisions_with_timers(
        ball,
        pegs,
        &state.neighbor_strategy,
        &state.config,
        _timers,
    );
}

/// Update peg physics and refresh spatial grid for optimal performance
///
/// This is the recommended function for Grid-based physics as it automatically
/// updates the spatial grid after peg positions change, maintaining optimal
/// collision detection performance.
///
/// # Performance Notes
///
/// - Peg force calculations: O(k×n) where k = neighbors per peg, n = total pegs
/// - Grid update: O(n) where n = total pegs
/// - Total complexity much better than O(n²) naive approach
///
/// # Examples
///
/// ```rust
/// use crate::scenes::game::physics;
/// use agb::fixnum::num;
///
/// let mut physics = physics::new(&pegs);
/// physics::update_peg_physics_with_grid(&mut pegs, num!(0.016), &mut physics)?;
/// ```
///
/// # Errors
///
/// Currently always returns `Ok(())` but may return errors in future versions
/// if physics constraints are violated (e.g., too many pegs, invalid positions).
#[must_use = "Physics update result should be checked for errors"]
pub fn update_peg_physics_with_grid(
    pegs: &mut Pegs,
    delta_time: Fixed,
    state: &mut PhysicsState<Grid>,
) -> Result<(), Error> {
    update_peg_physics(
        pegs,
        delta_time,
        &state.neighbor_strategy,
        &state.config,
    );
    state.update_grid(pegs);
    Ok(())
}

/// Update peg physics and refresh spatial grid with benchmarking support
#[must_use = "Physics update result should be checked for errors"]
pub fn update_peg_physics_with_grid_and_timers(
    pegs: &mut Pegs,
    delta_time: Fixed,
    state: &mut PhysicsState<Grid>,
    #[cfg(feature = "benchmark")] timers: Option<&agb::timer::Timers>,
    #[cfg(not(feature = "benchmark"))] _timers: Option<&agb::timer::Timers>,
) -> Result<(), Error> {
    #[cfg(feature = "benchmark")]
    update_peg_physics_with_timers(
        pegs,
        delta_time,
        &state.neighbor_strategy,
        &state.config,
        timers,
    );

    #[cfg(not(feature = "benchmark"))]
    update_peg_physics_with_timers(
        pegs,
        delta_time,
        &state.neighbor_strategy,
        &state.config,
        _timers,
    );

    state.update_grid(pegs);
    Ok(())
}

/// Generic peg physics update that works with any neighbor strategy
///
/// This function provides the same physics calculations as `update_peg_physics_with_grid`
/// but works with any implementation of `NeighborStrategy`, not just Grid.
///
/// Note: When using Grid strategy, prefer `update_peg_physics_with_grid` as it
/// automatically refreshes the spatial grid for optimal performance.
///
/// # Errors
///
/// Currently always returns `Ok(())` but may return errors in future versions.
#[must_use = "Physics update result should be checked for errors"]
pub fn update_peg_physics_generic<T: NeighborStrategy>(
    pegs: &mut Pegs,
    delta_time: Fixed,
    state: &PhysicsState<T>,
) -> Result<(), Error> {
    update_peg_physics(
        pegs,
        delta_time,
        &state.neighbor_strategy,
        &state.config,
    );
    Ok(())
}

