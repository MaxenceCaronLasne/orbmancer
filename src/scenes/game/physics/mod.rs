//! # Physics System
//! 
//! A high-performance physics engine optimized for peg-like games with spatial optimization.
//! 
//! ## Features
//! 
//! - **🎯 Ball Physics**: Realistic gravity, wall bouncing, and peg collisions
//! - **⚡ Spatial Optimization**: Grid-based neighbor finding for O(1) collision detection
//! - **🔧 Configurable**: Runtime-tunable physics parameters via [`PhysicsConfig`]
//! - **🚀 High Performance**: Optimized algorithms avoiding expensive operations
//! 
//! ## Performance Characteristics
//! 
//! | Operation | Complexity | Notes |
//! |-----------|------------|-------|
//! | Ball-peg collision | **O(k)** | k = nearby pegs (~9 typically) |
//! | Peg-peg forces | **O(k×n)** | k = neighbors per peg, n = total pegs |
//! | Grid updates | **O(n)** | n = total pegs |
//! | Ball physics | **O(1)** | Constant time per ball |
//! 
//! ## Quick Start
//! 
//! ```rust
//! use crate::scenes::game::physics;
//! use agb::fixnum::num;
//! 
//! // 1. Initialize physics with spatial optimization
//! let mut physics = physics::new(&pegs);
//! 
//! // 2. Each frame update loop:
//! loop {
//!     // Update ball (gravity, collisions, bouncing)
//!     physics::update_ball_physics(&mut ball, &mut pegs, delta_time, &physics);
//!     
//!     // Update pegs (forces, movement) and refresh spatial grid
//!     physics::update_peg_physics_with_grid(&mut pegs, delta_time, &mut physics)?;
//!     
//!     // Render frame...
//! }
//! ```
//! 
//! ## Customizing Physics
//! 
//! ```rust
//! use crate::scenes::game::physics::{PhysicsConfig, PhysicsState, Grid};
//! use agb::fixnum::num;
//! 
//! // Create custom physics configuration
//! let mut config = PhysicsConfig::default();
//! config.gravity_y = num!(150.0);              // Reduced gravity
//! config.peg_bounce_damping = num!(0.95);      // Less energy loss
//! config.interaction_force_strength = num!(300.0); // Stronger peg forces
//! 
//! // Create physics state with custom config
//! let grid = Grid::new(&pegs);
//! let physics = PhysicsState::with_config(grid, config);
//! ```
//! 
//! ## Architecture
//! 
//! The physics system is organized into focused modules:
//! 
//! - [`constants`] - Physics parameters and configuration
//! - [`collision`] - Ball collision detection (walls, pegs)  
//! - [`forces`] - Peg interaction forces and movement
//! - [`grid`] - Spatial optimization for performance
//! - [`state`] - Physics state management and main API
//! 
//! ## Optimization Details
//! 
//! ### Spatial Grid Optimization
//! The physics engine uses a spatial grid to avoid O(n²) collision checks:
//! - World is divided into grid cells of size 2³ = 8 units
//! - Each peg is mapped to its grid cell
//! - Collision detection only checks neighboring cells (typically 9 cells)
//! - Results in ~50x speedup for collision detection with many pegs
//! 
//! ### Mathematical Optimizations
//! - **Squared distances**: Avoids expensive √ operations until collision confirmed
//! - **Pre-computed values**: Force radii squared, collision distances cached  
//! - **Early termination**: Distance checks exit early when objects too far apart
//! - **Inline functions**: Critical path functions marked `#[inline]` for zero-cost abstraction

pub mod constants;
pub mod collision;
pub mod forces;
pub mod grid;
pub mod state;

#[cfg(feature = "benchmark")]
pub mod bench;

// Tests removed - AGB framework doesn't support standard test crate in no_std environment

// Re-export main API for users
pub use constants::{
    PhysicsConfig, Distance, Time, ForceStrength, DampingFactor
};
pub use grid::{Grid, NeighborStrategy};
pub use state::{
    PhysicsState, new, update_ball_physics, update_ball_physics_with_timers,
    update_peg_physics_with_grid, update_peg_physics_with_grid_and_timers, 
    update_peg_physics_generic
};