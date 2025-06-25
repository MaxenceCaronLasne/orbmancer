#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::fixnum::{num, vec2};

mod ball;
mod error;
mod types;

extern crate alloc;

fn main(mut gba: agb::Gba) -> Result<(), error::Error> {
    let mut gfx = gba.graphics.get();

    let mut b = ball::Ball::new(vec2(num!(0), num!(0)));

    loop {
        let mut frame = gfx.frame();
        b.show(&mut frame);
        frame.commit();
    }
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
