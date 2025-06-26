use crate::error::Error;
use crate::scenes::Scene;
use crate::types::Fixed;
use agb::fixnum::{num, vec2};
use agb::input::{Button, ButtonController};
use alloc::alloc::Global;
use alloc::vec::Vec;
use ball::Ball;
use peg::Peg;

pub mod ball;
pub mod peg;
pub mod physics;

enum State {
    Aiming,
    Falling,
    Counting,
}

fn spawn_pegs(pegs: &mut Vec<Peg>) {
    for i in 1..10 {
        for j in 1..20 {
            pegs.push(Peg::new(vec2(Fixed::new(j * 20), Fixed::new(i * 20))));
        }
    }
}

fn update_aiming(input: &mut ButtonController) -> Result<State, Error> {
    if input.is_just_pressed(Button::A) {
        return Ok(State::Falling);
    }

    Ok(State::Aiming)
}

fn update_falling(ball: &mut Ball, pegs: &mut [Peg]) -> Result<State, Error> {
    physics::move_and_collide(ball, pegs, num!(1) / num!(60));

    if ball.position.y > num!(180) {
        return Ok(State::Counting);
    }

    Ok(State::Falling)
}

fn update_counting(
    ball: &mut Ball,
    pegs: &mut Vec<Peg>,
) -> Result<State, Error> {
    ball.position = vec2(num!(21), num!(0));
    pegs.retain(|p| !p.is_touched());
    Ok(State::Aiming)
}

pub fn main(gba: &mut agb::Gba) -> Result<Scene, Error> {
    let mut state = State::Aiming;

    let mut gfx = gba.graphics.get();
    let mut input = ButtonController::new();

    let mut ball = Ball::new(vec2(num!(21), num!(0)));
    let mut pegs = Vec::<Peg>::new_in(Global);

    spawn_pegs(&mut pegs);

    loop {
        input.update();

        state = match state {
            State::Aiming => update_aiming(&mut input)?,
            State::Falling => update_falling(&mut ball, &mut pegs)?,
            State::Counting => update_counting(&mut ball, &mut pegs)?,
        };

        let mut frame = gfx.frame();
        ball.show(&mut frame);
        for p in pegs.iter_mut() {
            p.show(&mut frame);
        }
        frame.commit();
    }
}
