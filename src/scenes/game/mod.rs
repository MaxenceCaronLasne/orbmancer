use crate::error::Error;
use crate::scenes::State;
use agb::fixnum::{num, vec2};

mod ball;

pub fn main(gba: &mut agb::Gba) -> Result<State, Error> {
    let mut gfx = gba.graphics.get();

    let mut b = ball::Ball::new(vec2(num!(0), num!(0)));

    loop {
        let mut frame = gfx.frame();
        b.show(&mut frame);
        frame.commit();
    }
}
