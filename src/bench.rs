use crate::error::Error;

const LOG_FREQUENCY: u16 = 1;
const MAX_NESTED_TIMERS: usize = 8;

static mut BENCHMARK: Benchmark = Benchmark {
    before: None,
    after: None,
    log_counter: 0,
    nested_timers: [None; MAX_NESTED_TIMERS],
    current_timer_depth: 0,
    stats: PhysicsStats::new(),
};

struct Benchmark {
    before: Option<(u16, u16)>,
    after: Option<(u16, u16)>,
    log_counter: u16,
    nested_timers: [Option<(u16, u16)>; MAX_NESTED_TIMERS],
    current_timer_depth: usize,
    stats: PhysicsStats,
}

#[derive(Clone, Copy)]
pub struct PhysicsStats {
    total_frame_time: u32,
    grid_query_time: u32,
    collision_time: u32,
    force_calculation_time: u32,
    peg_update_time: u32,
    frame_count: u32,
}

impl PhysicsStats {
    const fn new() -> Self {
        Self {
            total_frame_time: 0,
            grid_query_time: 0,
            collision_time: 0,
            force_calculation_time: 0,
            peg_update_time: 0,
            frame_count: 0,
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn add_measurement(&mut self, operation: PhysicsOperation, time: u32) {
        match operation {
            PhysicsOperation::TotalFrame => self.total_frame_time = self.total_frame_time.saturating_add(time),
            PhysicsOperation::GridQuery => self.grid_query_time = self.grid_query_time.saturating_add(time),
            PhysicsOperation::Collision => self.collision_time = self.collision_time.saturating_add(time),
            PhysicsOperation::ForceCalculation => self.force_calculation_time = self.force_calculation_time.saturating_add(time),
            PhysicsOperation::PegUpdate => self.peg_update_time = self.peg_update_time.saturating_add(time),
        }
        self.frame_count = self.frame_count.saturating_add(1);
    }
}

#[derive(Clone, Copy)]
pub enum PhysicsOperation {
    TotalFrame,
    GridQuery,
    Collision,
    ForceCalculation,
    PegUpdate,
}

pub fn reset(timers: &mut agb::timer::Timers) {
    unsafe {
        BENCHMARK.before = None;
        BENCHMARK.after = None;
    }

    timers.timer3.set_cascade(true);
    timers.timer3.set_enabled(true);
    timers.timer2.set_enabled(true);
}

pub fn set_before(timers: &agb::timer::Timers) -> Result<(), Error> {
    unsafe {
        match (BENCHMARK.before, BENCHMARK.after) {
            (None, None) => {
                BENCHMARK.before =
                    Some((timers.timer3.value(), timers.timer2.value()));
                Ok(())
            }
            _ => Err(Error::BenchmarkError),
        }
    }
}

pub fn set_after(timers: &agb::timer::Timers) -> Result<(), Error> {
    unsafe {
        match (BENCHMARK.before, BENCHMARK.after) {
            (Some((_, _)), None) => {
                BENCHMARK.after =
                    Some((timers.timer3.value(), timers.timer2.value()));
                Ok(())
            }
            _ => Err(Error::BenchmarkError),
        }
    }
}

fn get_result() -> Result<u32, Error> {
    unsafe {
        match (BENCHMARK.before, BENCHMARK.after) {
            (
                Some((before_long, before_short)),
                Some((after_long, after_short)),
            ) => {
                let before: u32 =
                    ((before_long as u32) << 16) + (before_short as u32);
                let after: u32 =
                    ((after_long as u32) << 16) + (after_short as u32);
                Ok(after.wrapping_sub(before))
            }
            _ => Err(Error::BenchmarkError),
        }
    }
}

pub fn log(tag: &str) -> Result<(), Error> {
    let result = get_result()?;

    unsafe {
        if BENCHMARK.log_counter == 0 || BENCHMARK.log_counter == 1 {
            agb::println!("[BENCH][{}] {}", tag, result)
        }
        BENCHMARK.log_counter = (BENCHMARK.log_counter + 1) % LOG_FREQUENCY;
    };

    Ok(())
}

/// Start a nested timer for detailed physics operation measurement
pub fn start_nested_timer(timers: &agb::timer::Timers) -> Result<(), Error> {
    unsafe {
        if BENCHMARK.current_timer_depth >= MAX_NESTED_TIMERS {
            return Err(Error::BenchmarkError);
        }
        
        BENCHMARK.nested_timers[BENCHMARK.current_timer_depth] = 
            Some((timers.timer3.value(), timers.timer2.value()));
        BENCHMARK.current_timer_depth += 1;
        Ok(())
    }
}

/// End a nested timer and return the elapsed time
pub fn end_nested_timer(timers: &agb::timer::Timers) -> Result<u32, Error> {
    unsafe {
        if BENCHMARK.current_timer_depth == 0 {
            return Err(Error::BenchmarkError);
        }
        
        BENCHMARK.current_timer_depth -= 1;
        let start_time = BENCHMARK.nested_timers[BENCHMARK.current_timer_depth]
            .ok_or(Error::BenchmarkError)?;
        
        let before: u32 = ((start_time.0 as u32) << 16) + (start_time.1 as u32);
        let after: u32 = ((timers.timer3.value() as u32) << 16) + (timers.timer2.value() as u32);
        
        Ok(after.wrapping_sub(before))
    }
}

/// Record a physics operation measurement
pub fn record_physics_measurement(operation: PhysicsOperation, time: u32) {
    unsafe {
        let benchmark = &raw mut BENCHMARK;
        (*benchmark).stats.add_measurement(operation, time);
    }
}

/// Get current physics statistics
pub fn get_physics_stats() -> PhysicsStats {
    unsafe {
        BENCHMARK.stats
    }
}

/// Reset physics statistics
pub fn reset_physics_stats() {
    unsafe {
        let benchmark = &raw mut BENCHMARK;
        (*benchmark).stats.reset();
    }
}

/// Log detailed physics performance breakdown
pub fn log_physics_breakdown() {
    unsafe {
        let benchmark = &raw const BENCHMARK;
        let stats = &(*benchmark).stats;
        if stats.frame_count > 0 {
            let avg_total = stats.total_frame_time / stats.frame_count;
            let avg_grid = stats.grid_query_time / stats.frame_count;
            let avg_collision = stats.collision_time / stats.frame_count;
            let avg_force = stats.force_calculation_time / stats.frame_count;
            let avg_peg = stats.peg_update_time / stats.frame_count;
            
            agb::println!("[PHYSICS] Total:{} Grid:{} Collision:{} Force:{} Peg:{}", 
                         avg_total, avg_grid, avg_collision, avg_force, avg_peg);
        }
    }
}

/// Conditional compilation macros for benchmarking
#[cfg(feature = "benchmark")]
pub use bench_macros::*;

#[cfg(not(feature = "benchmark"))]
pub mod bench_macros {
    #[macro_export]
    macro_rules! bench_start {
        ($timers:expr) => {};
    }
    
    #[macro_export]
    macro_rules! bench_end {
        ($timers:expr, $operation:expr) => {};
    }
    
    #[macro_export]
    macro_rules! bench_scope {
        ($timers:expr, $operation:expr, $code:block) => {
            $code
        };
    }
}

#[cfg(feature = "benchmark")]
pub mod bench_macros {
    #[macro_export]
    macro_rules! bench_start {
        ($timers:expr) => {
            crate::bench::start_nested_timer($timers).ok();
        };
    }
    
    #[macro_export]
    macro_rules! bench_end {
        ($timers:expr, $operation:expr) => {
            if let Ok(time) = crate::bench::end_nested_timer($timers) {
                crate::bench::record_physics_measurement($operation, time);
            }
        };
    }
    
    #[macro_export]
    macro_rules! bench_scope {
        ($timers:expr, $operation:expr, $code:block) => {{
            crate::bench::start_nested_timer($timers).ok();
            let result = $code;
            if let Ok(time) = crate::bench::end_nested_timer($timers) {
                crate::bench::record_physics_measurement($operation, time);
            }
            result
        }};
    }
}
