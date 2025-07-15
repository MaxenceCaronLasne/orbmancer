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
mod position;
mod save;
mod scenes;
mod text;

pub type Fixed = FixedNum<8>;
pub type Coordinates = Vector2D<Fixed>;
pub type Force = Vector2D<Fixed>;

const DELTA: f32 = 1.0 / 60.0;

#[cfg(not(test))]
#[agb::entry]
fn entry(gba: agb::Gba) -> ! {
    agb::println!("Starting game...");
    let save = save::Save::new();

    match scenes::main(gba, save) {
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
    fn test_frame_commit(gba: &mut Gba) {
        let mut gfx = gba.graphics.get();
        let mut frame = gfx.frame();
        frame.commit();
    }

    #[test_case]
    fn test_physics_constants(_gba: &mut Gba) {
        use crate::scenes::game::ball::RADIUS as BALL_RADIUS;
        use crate::scenes::game::peg::RADIUS as PEG_RADIUS;

        agb::println!("Testing physics constants");

        // Test that physics constants are reasonable
        assert!(PEG_RADIUS > 0);
        assert!(BALL_RADIUS > 0);
        assert!(PEG_RADIUS < 20); // Reasonable size for GBA screen
        assert!(BALL_RADIUS < 20); // Reasonable size for GBA screen

        agb::println!(
            "Peg radius: {}, Ball radius: {}",
            PEG_RADIUS,
            BALL_RADIUS
        );
        agb::println!("Physics constants: OK");
    }

    #[test_case]
    fn test_coordinates_math(_gba: &mut Gba) {
        use crate::{Coordinates, Fixed};
        use agb::fixnum::vec2;

        agb::println!("Testing coordinate math");

        let pos1: Coordinates = vec2(Fixed::new(10), Fixed::new(20));
        let pos2: Coordinates = vec2(Fixed::new(5), Fixed::new(15));
        let result = pos1 + pos2;

        agb::println!("Position 1: {:?}", pos1);
        agb::println!("Position 2: {:?}", pos2);
        agb::println!("Sum: {:?}", result);

        // Test that coordinate math works correctly
        assert_eq!(result.x, Fixed::new(15));
        assert_eq!(result.y, Fixed::new(35));

        agb::println!("Coordinate math: OK");
    }

    #[test_case]
    fn test_save_system(_gba: &mut Gba) {
        use crate::save::{BallKind, Save};

        agb::println!("Testing save system");

        let mut save = Save::new();
        assert!(save.inventory().is_empty());

        let result = save.push_ball(BallKind::Identity);
        assert!(result.is_ok());
        assert_eq!(save.inventory().len(), 1);

        let result = save.push_ball(BallKind::TheDoubler);
        assert!(result.is_ok());
        assert_eq!(save.inventory().len(), 2);

        agb::println!(
            "Save system: OK - {} balls stored",
            save.inventory().len()
        );
    }
}
