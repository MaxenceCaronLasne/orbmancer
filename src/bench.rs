static mut GRID_START: Option<(u16, u16)> = None;
static mut COLLISION_START: Option<(u16, u16)> = None;
static mut TIMERS: Option<*const agb::timer::Timers> = None;
static mut GRID_TIME: u32 = 0;
static mut COLLISION_TIME: u32 = 0;
static mut FRAME_BEFORE: Option<(u16, u16)> = None;
static mut FRAME_AFTER: Option<(u16, u16)> = None;

fn start(timers: &agb::timer::Timers) -> (u16, u16) {
    (timers.timer3.value(), timers.timer2.value())
}

fn stop(start_time: (u16, u16), timers: &agb::timer::Timers) -> u32 {
    let after = (timers.timer3.value(), timers.timer2.value());
    let before_val: u32 = ((start_time.0 as u32) << 16) + (start_time.1 as u32);
    let after_val: u32 = ((after.0 as u32) << 16) + (after.1 as u32);
    after_val.wrapping_sub(before_val)
}

pub fn reset(timers: &mut agb::timer::Timers) {
    unsafe {
        GRID_TIME = 0;
        COLLISION_TIME = 0;
        FRAME_BEFORE = None;
        FRAME_AFTER = None;
        TIMERS = Some(core::mem::transmute(timers as *const _));
    }
    timers.timer3.set_cascade(true);
    timers.timer3.set_enabled(true);
    timers.timer2.set_enabled(true);
}

pub fn set_timers(timers: &agb::timer::Timers) {
    unsafe {
        TIMERS = Some(core::mem::transmute(timers as *const _));
    }
}

pub fn set_before(timers: &agb::timer::Timers) {
    unsafe {
        FRAME_BEFORE = Some(start(timers));
    }
}

pub fn set_after(timers: &agb::timer::Timers) {
    unsafe {
        FRAME_AFTER = Some(start(timers));
    }
}

pub fn log(tag: &str) {
    unsafe {
        if let (Some(before), Some(after)) = (FRAME_BEFORE, FRAME_AFTER) {
            let before_val: u32 = ((before.0 as u32) << 16) + (before.1 as u32);
            let after_val: u32 = ((after.0 as u32) << 16) + (after.1 as u32);
            let result = after_val.wrapping_sub(before_val);
            
            if tag == "TOTAL_PHYSICS" {
                let grid_time = GRID_TIME;
                let collision_time = COLLISION_TIME;
                agb::println!("[BENCH][{}] {} [Grid:{} Collision:{}]", tag, result, grid_time, collision_time);
            } else {
                agb::println!("[BENCH][{}] {}", tag, result);
            }
        }
    }
}

#[cfg(feature = "benchmark")]
pub fn bench_start_grid() {
    unsafe {
        if let Some(timers_ptr) = TIMERS {
            let timers = &*timers_ptr;
            GRID_START = Some(start(timers));
        }
    }
}

#[cfg(feature = "benchmark")]
pub fn bench_end_grid() {
    unsafe {
        if let Some(timers_ptr) = TIMERS {
            if let Some(start_time) = GRID_START {
                let timers = &*timers_ptr;
                let elapsed = stop(start_time, timers);
                GRID_TIME = GRID_TIME.saturating_add(elapsed);
            }
        }
    }
}

#[cfg(feature = "benchmark")]
pub fn bench_start_collision() {
    unsafe {
        if let Some(timers_ptr) = TIMERS {
            let timers = &*timers_ptr;
            COLLISION_START = Some(start(timers));
        }
    }
}

#[cfg(feature = "benchmark")]
pub fn bench_end_collision() {
    unsafe {
        if let Some(timers_ptr) = TIMERS {
            if let Some(start_time) = COLLISION_START {
                let timers = &*timers_ptr;
                let elapsed = stop(start_time, timers);
                COLLISION_TIME = COLLISION_TIME.saturating_add(elapsed);
            }
        }
    }
}

#[cfg(not(feature = "benchmark"))]
pub fn bench_start_grid() {}

#[cfg(not(feature = "benchmark"))]
pub fn bench_end_grid() {}

#[cfg(not(feature = "benchmark"))]
pub fn bench_start_collision() {}

#[cfg(not(feature = "benchmark"))]
pub fn bench_end_collision() {}
