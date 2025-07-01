#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
#![feature(allocator_api)]

use agb::fixnum::{FixedNum, Vector2D};

extern crate alloc;

mod bench;
mod error;
mod physics;
mod scenes;

#[cfg(test)]
mod test_scenes;

pub type Fixed = FixedNum<8>;
pub type Coordinates = Vector2D<Fixed>;
pub type Force = Vector2D<Fixed>;

#[cfg(not(test))]
#[agb::entry]
fn entry(gba: agb::Gba) -> ! {
    match scenes::main(gba) {
        Ok(()) => {
            agb::println!("Quitting...");
        }
        Err(err) => {
            agb::eprintln!("Main loop exited with error: {:?}", err);
        }
    }

    loop {
        agb::halt();
    }
}

#[cfg(test)]
#[agb::entry]
fn main_test(mut _gba: agb::Gba) -> ! {
    test_main();
    loop {
        agb::display::busy_wait_for_vblank();
    }
}

#[cfg(test)]
mod tests {
    use agb::{Gba, fixnum::FixedNum};

    #[test_case]
    fn test_sample(_gba: &mut Gba) {}

    #[test_case]
    fn test_fixnum(_gba: &mut Gba) {
        let radius = FixedNum::<8>::new(3);
        agb::println!("radius={}, raw={:#08x}", radius, radius.to_raw());
    }

    #[test_case]
    fn test_physics_sparse_pegs(gba: &mut Gba) {
        use crate::test_scenes::{PhysicsTest, run_test_scene};

        agb::println!("Running physics test: sparse pegs");
        agb::println!("Controls: L/R/SELECT = switch test, START = finish");

        match run_test_scene::<PhysicsTest>(gba, 3600) {
            // 60 seconds max
            Ok(result) => agb::println!("Test result: {:?}", result),
            Err(e) => agb::println!("Test error: {:?}", e),
        }
    }

    #[test_case]
    fn test_physics_all_scenarios(gba: &mut Gba) {
        agb::println!("Running comprehensive physics test");
        agb::println!(
            "Controls: L=sparse pegs, R=dense cluster, SELECT=wall bounce, START=finish"
        );

        match crate::test_scenes::run_test_scene::<
            crate::test_scenes::PhysicsTest,
        >(gba, 3600)
        {
            Ok(result) => agb::println!("Test result: {:?}", result),
            Err(e) => agb::println!("Test error: {:?}", e),
        }
    }
}
