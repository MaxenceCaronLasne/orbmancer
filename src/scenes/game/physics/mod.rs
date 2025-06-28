use crate::scenes::game::peg::Pegs;
use crate::scenes::game::{
    ball::Ball, physics::collision::handle_peg_peg_collisions,
};
use crate::types::Fixed;
use agb::fixnum::{num, vec2};
use collision::{handle_ball_peg_collisions, handle_ball_wall_collisions};
use constants::{SCREEN_BOTTOM, ZERO, PEG_MIN_VELOCITY};

pub mod collision;
pub mod constants;
pub mod grid;
pub mod state;

pub use state::PhysicsState;

pub fn update_ball(
    ball: &mut Ball,
    pegs: &mut Pegs,
    delta_time: Fixed,
    state: &mut PhysicsState,
) {
    let initial_position = ball.position;
    let initial_velocity = ball.velocity;

    ball.velocity =
        initial_velocity + vec2(num!(0), state.config.gravity_y) * delta_time;

    ball.position = initial_position + ball.velocity * delta_time;

    handle_ball_wall_collisions(ball);

    if ball.position.y > num!(SCREEN_BOTTOM) {
        ball.velocity = vec2(num!(ZERO), num!(ZERO));
        return;
    }

    handle_ball_peg_collisions(ball, pegs, state);
}

pub fn update_pegs(
    pegs: &mut Pegs,
    delta_time: Fixed,
    state: &mut PhysicsState,
) {
    for i in 0..pegs.count {
        pegs.set_position(i, pegs.position(i) + pegs.velocity(i) * delta_time);
        let damped_velocity = pegs.velocity(i) * state.config().peg_velocity_damping;
        if damped_velocity.magnitude_squared() < num!(PEG_MIN_VELOCITY * PEG_MIN_VELOCITY) {
            pegs.set_velocity(i, vec2(num!(0), num!(0)));
        } else {
            pegs.set_velocity(i, damped_velocity);
        }
    }

    state.update_grid_selective(pegs);
    handle_peg_peg_collisions(pegs, state);
}
