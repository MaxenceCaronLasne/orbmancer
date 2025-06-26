#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
#![feature(allocator_api)]
extern crate alloc;

mod error;
mod scenes;
mod types;

#[cfg(test)]
mod test_scenes;

fn main(gba: agb::Gba) -> Result<(), error::Error> {
    scenes::main(gba)
}

#[agb::entry]
fn entry(gba: agb::Gba) -> ! {
    match main(gba) {
        Ok(()) => {}
        Err(err) => {
            agb::eprintln!("Main loop exited with error: {:?}", err);
        }
    }

    loop {
        agb::halt();
    }
}
