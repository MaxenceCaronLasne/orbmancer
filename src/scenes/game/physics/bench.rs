//! Physics-specific benchmarking utilities
//! 
//! This module provides convenient wrappers around the global benchmarking 
//! system specifically tailored for physics operations.

#[cfg(feature = "benchmark")]
use crate::bench::{PhysicsOperation, start_nested_timer, end_nested_timer, record_physics_measurement};

/// Benchmarking wrapper for physics operations
pub struct PhysicsBench;

#[cfg(feature = "benchmark")]
impl PhysicsBench {
    /// Measure grid neighbor query performance
    pub fn measure_grid_query<F, R>(timers: &agb::timer::Timers, f: F) -> R 
    where 
        F: FnOnce() -> R 
    {
        let _ = start_nested_timer(timers);
        let result = f();
        if let Ok(time) = end_nested_timer(timers) {
            record_physics_measurement(PhysicsOperation::GridQuery, time);
        }
        result
    }

    /// Measure collision detection performance
    pub fn measure_collision<F, R>(timers: &agb::timer::Timers, f: F) -> R 
    where 
        F: FnOnce() -> R 
    {
        let _ = start_nested_timer(timers);
        let result = f();
        if let Ok(time) = end_nested_timer(timers) {
            record_physics_measurement(PhysicsOperation::Collision, time);
        }
        result
    }

    /// Measure force calculation performance
    pub fn measure_force_calculation<F, R>(timers: &agb::timer::Timers, f: F) -> R 
    where 
        F: FnOnce() -> R 
    {
        let _ = start_nested_timer(timers);
        let result = f();
        if let Ok(time) = end_nested_timer(timers) {
            record_physics_measurement(PhysicsOperation::ForceCalculation, time);
        }
        result
    }

    /// Measure peg update performance
    pub fn measure_peg_update<F, R>(timers: &agb::timer::Timers, f: F) -> R 
    where 
        F: FnOnce() -> R 
    {
        let _ = start_nested_timer(timers);
        let result = f();
        if let Ok(time) = end_nested_timer(timers) {
            record_physics_measurement(PhysicsOperation::PegUpdate, time);
        }
        result
    }

    /// Measure total frame performance
    pub fn measure_total_frame<F, R>(timers: &agb::timer::Timers, f: F) -> R 
    where 
        F: FnOnce() -> R 
    {
        let _ = start_nested_timer(timers);
        let result = f();
        if let Ok(time) = end_nested_timer(timers) {
            record_physics_measurement(PhysicsOperation::TotalFrame, time);
        }
        result
    }
}

#[cfg(not(feature = "benchmark"))]
impl PhysicsBench {
    #[inline(always)]
    pub fn measure_grid_query<F, R>(_timers: &agb::timer::Timers, f: F) -> R 
    where 
        F: FnOnce() -> R 
    {
        f()
    }

    #[inline(always)]
    pub fn measure_collision<F, R>(_timers: &agb::timer::Timers, f: F) -> R 
    where 
        F: FnOnce() -> R 
    {
        f()
    }

    #[inline(always)]
    pub fn measure_force_calculation<F, R>(_timers: &agb::timer::Timers, f: F) -> R 
    where 
        F: FnOnce() -> R 
    {
        f()
    }

    #[inline(always)]
    pub fn measure_peg_update<F, R>(_timers: &agb::timer::Timers, f: F) -> R 
    where 
        F: FnOnce() -> R 
    {
        f()
    }

    #[inline(always)]
    pub fn measure_total_frame<F, R>(_timers: &agb::timer::Timers, f: F) -> R 
    where 
        F: FnOnce() -> R 
    {
        f()
    }
}