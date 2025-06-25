use crate::error::Error;
use crate::scenes::State;
use crate::types::Fixed;
use agb::fixnum::{num, vec2};
use alloc::alloc::Global;
use alloc::vec::Vec;
use ball::Ball;
use peg::Peg;

mod ball;
mod peg;

fn spawn_pegs(pegs: &mut Vec<Peg>) {
    for i in 1..10 {
        for j in 1..20 {
            pegs.push(Peg::new(vec2(Fixed::new(j * 10), Fixed::new(i * 10))));
        }
    }
}

pub fn main(gba: &mut agb::Gba) -> Result<State, Error> {
    let mut gfx = gba.graphics.get();

    let mut ball = Ball::new(vec2(num!(0), num!(0)));
    let mut pegs = Vec::<Peg>::new_in(Global);

    spawn_pegs(&mut pegs);

    loop {
        let mut frame = gfx.frame();

        ball.show(&mut frame);
        for p in pegs.iter_mut() {
            p.show(&mut frame);
        }

        frame.commit();
    }
}
