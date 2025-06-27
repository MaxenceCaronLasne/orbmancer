use crate::error::Error;

const LOG_FREQUENCY: u16 = 100;

static mut BENCHMARK: Benchmark = Benchmark {
    before: None,
    after: None,
    log_counter: 0,
};

struct Benchmark {
    before: Option<(u16, u16)>,
    after: Option<(u16, u16)>,
    log_counter: u16,
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
                Ok(after - before)
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
