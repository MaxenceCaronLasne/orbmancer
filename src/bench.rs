use agb::timer::Timers;

const MAX_BENCHMARKS: usize = 16;
const MAX_STACK_DEPTH: usize = 8;

#[derive(Copy, Clone)]
struct BenchEntry {
    tag: &'static str,
    total_time: u32,
    count: u32,
}

#[derive(Copy, Clone)]
struct TimerFrame {
    tag: &'static str,
    start_time: (u16, u16),
}

static mut TIMERS: Option<*const Timers> = None;
static mut BENCHMARKS: [Option<BenchEntry>; MAX_BENCHMARKS] =
    [None; MAX_BENCHMARKS];
static mut TIMER_STACK: [Option<TimerFrame>; MAX_STACK_DEPTH] =
    [None; MAX_STACK_DEPTH];
static mut STACK_TOP: usize = 0;

pub fn init(timers: &mut Timers) {
    unsafe {
        TIMERS = Some(core::mem::transmute(timers as *const _));
        for i in 0..MAX_BENCHMARKS {
            BENCHMARKS[i] = None;
        }
        for i in 0..MAX_STACK_DEPTH {
            TIMER_STACK[i] = None;
        }
        STACK_TOP = 0;
    }

    timers.timer3.set_cascade(true);
    timers.timer3.set_enabled(true);
    timers.timer2.set_enabled(true);
}

#[cfg(feature = "benchmark")]
pub fn start(tag: &'static str) {
    unsafe {
        if let Some(timers_ptr) = TIMERS {
            let timers = &*timers_ptr;
            let start_time = (timers.timer3.value(), timers.timer2.value());

            if STACK_TOP < MAX_STACK_DEPTH {
                TIMER_STACK[STACK_TOP] = Some(TimerFrame { tag, start_time });
                STACK_TOP += 1;
            } else {
                agb::println!("[BENCH] Stack overflow on start({})", tag);
            }
        }
    }
}

#[cfg(feature = "benchmark")]
pub fn stop(tag: &'static str) {
    unsafe {
        if let Some(timers_ptr) = TIMERS {
            let timers = &*timers_ptr;
            let end_time = (timers.timer3.value(), timers.timer2.value());

            if STACK_TOP > 0 {
                STACK_TOP -= 1;
                if let Some(frame) = &TIMER_STACK[STACK_TOP] {
                    if frame.tag == tag {
                        let elapsed = calc_elapsed(frame.start_time, end_time);
                        add_to_bench(tag, elapsed);
                    } else {
                        agb::println!("[BENCH] Mismatched stop({}) expected ({})", tag, frame.tag);
                    }
                    TIMER_STACK[STACK_TOP] = None;
                } else {
                    agb::println!("[BENCH] Empty stack on stop({})", tag);
                }
            } else {
                agb::println!("[BENCH] Stack underflow on stop({})", tag);
            }
        }
    }
}

#[cfg(feature = "benchmark")]
pub fn log() {
    unsafe {
        agb::println!("[BENCH] Results:");
        for i in 0..MAX_BENCHMARKS {
            if let Some(entry) = BENCHMARKS[i] {
                if entry.count > 0 {
                    let avg = entry.total_time / entry.count;
                    agb::println!(
                        "  {}: {} total, {} calls, {} avg",
                        entry.tag,
                        entry.total_time,
                        entry.count,
                        avg
                    );
                }
            }
        }

        for i in 0..MAX_BENCHMARKS {
            BENCHMARKS[i] = None;
        }
    }
}

#[cfg(feature = "benchmark")]
fn calc_elapsed(start: (u16, u16), end: (u16, u16)) -> u32 {
    let start_val: u32 = ((start.0 as u32) << 16) + (start.1 as u32);
    let end_val: u32 = ((end.0 as u32) << 16) + (end.1 as u32);
    
    let elapsed = end_val.wrapping_sub(start_val);
    
    // Check for unrealistic values (likely timer overflow/underflow)
    if elapsed > 1_000_000 {  // More than ~1M cycles is suspicious for short operations
        0  // Return 0 for obviously invalid measurements
    } else {
        elapsed
    }
}

#[cfg(feature = "benchmark")]
fn add_to_bench(tag: &'static str, elapsed: u32) {
    unsafe {
        for i in 0..MAX_BENCHMARKS {
            if let Some(entry) = &mut BENCHMARKS[i] {
                if entry.tag == tag {
                    entry.total_time = entry.total_time.saturating_add(elapsed);
                    entry.count += 1;
                    return;
                }
            }
        }

        for i in 0..MAX_BENCHMARKS {
            if BENCHMARKS[i].is_none() {
                BENCHMARKS[i] = Some(BenchEntry {
                    tag,
                    total_time: elapsed,
                    count: 1,
                });
                return;
            }
        }
    }
}

#[cfg(not(feature = "benchmark"))]
pub fn start(_tag: &'static str) {}

#[cfg(not(feature = "benchmark"))]
pub fn stop(_tag: &'static str) {}

#[cfg(not(feature = "benchmark"))]
pub fn log() {}

