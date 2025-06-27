use crate::error::Error;
use crate::scenes::game::{Ball, peg::Pegs};
use crate::types::Fixed;
use agb::fixnum::{num, vec2};

use super::constants::{PhysicsConfig, SCREEN_BOTTOM, ZERO};
use super::collision::{handle_ball_wall_collisions, handle_ball_peg_collisions};
use super::forces::update_peg_physics;
use super::grid::{Grid, NeighborStrategy};

/// Physics state that manages spatial optimization strategy
/// 
/// Generic over neighbor strategy to allow different spatial optimization
/// approaches (e.g., Grid, QuadTree, etc.)
pub struct PhysicsState<T: NeighborStrategy> {
    neighbor_strategy: T,
    config: PhysicsConfig,
}

impl<T: NeighborStrategy> PhysicsState<T> {
    /// Create a new physics state with default configuration
    #[must_use]
    pub fn new(neighbor_strategy: T) -> Self {
        Self { 
            neighbor_strategy,
            config: PhysicsConfig::default(),
        }
    }

    /// Create a new physics state with custom configuration
    #[must_use]
    pub fn with_config(neighbor_strategy: T, config: PhysicsConfig) -> Self {
        Self { neighbor_strategy, config }
    }

    /// Get read-only access to the physics configuration
    #[must_use]
    pub fn config(&self) -> &PhysicsConfig {
        &self.config
    }

    /// Get mutable access to the physics configuration
    pub fn config_mut(&mut self) -> &mut PhysicsConfig {
        &mut self.config
    }
}

/// Grid-specific physics state with additional optimization methods
impl PhysicsState<Grid> {
    /// Update the spatial grid after peg positions change
    /// 
    /// This should be called after peg physics updates to maintain
    /// spatial optimization accuracy for ball-peg collision detection.
    pub fn update_grid(&mut self, pegs: &Pegs) {
        self.neighbor_strategy.update(pegs);
    }
}

/// Create a new physics state with Grid-based spatial optimization
/// 
/// This is the recommended way to create a physics state for most use cases.
/// The Grid provides excellent performance for typical peg counts (10-50 pegs).
/// 
/// # Examples
/// 
/// ```rust
/// use crate::scenes::game::physics;
/// 
/// let physics = physics::new(&pegs);
/// ```
#[must_use]
pub fn new(pegs: &Pegs) -> PhysicsState<Grid> {
    PhysicsState::new(Grid::new(pegs))
}

/// Update ball physics including gravity, wall collisions, and peg collisions
/// 
/// # Physics Steps
/// 1. Apply gravity acceleration
/// 2. Update position using velocity
/// 3. Handle wall collisions 
/// 4. Check for ball falling below screen
/// 5. Handle peg collisions using spatial optimization
pub fn update_ball_physics<T: NeighborStrategy>(
    ball: &mut Ball,
    pegs: &mut Pegs,
    delta_time: Fixed,
    state: &PhysicsState<T>,
) {
    let initial_position = ball.position;
    let initial_velocity = ball.velocity;

    // Apply gravity and update position
    ball.velocity = initial_velocity + vec2(num!(0), state.config.gravity_y) * delta_time;
    ball.position = initial_position + ball.velocity * delta_time;

    handle_ball_wall_collisions(ball);

    // Reset ball if it falls below screen
    if ball.position.y > num!(SCREEN_BOTTOM) {
        ball.velocity = vec2(num!(ZERO), num!(ZERO));
        return;
    }

    handle_ball_peg_collisions(ball, pegs, &state.neighbor_strategy, &state.config);
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
    update_peg_physics(pegs, delta_time, &state.neighbor_strategy, &state.config);
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
    update_peg_physics(pegs, delta_time, &state.neighbor_strategy, &state.config);
    Ok(())
}