use crate::error::Error;
use crate::scenes::Scene;
use crate::scenes::game::physics::PhysicsState;
use crate::types::Fixed;
use agb::fixnum::{num, vec2};
use agb::input::{Button, ButtonController};
use agb::rng::RandomNumberGenerator;
use ball::Ball;
use peg::Pegs;

pub mod ball;
pub mod peg;
pub mod physics;

// Game constants
const MAX_HORIZONTAL_VELOCITY: f32 = 100.0; // Maximum horizontal velocity
const VELOCITY_CHANGE_RATE: f32 = 120.0; // Velocity change per second
const DELTA_TIME: f32 = 1.0 / 60.0; // 60 FPS
const BALL_START_X: f32 = 21.0;
const BALL_START_Y: f32 = 0.0;
const SCREEN_BOTTOM: f32 = 180.0;

enum State {
    Aiming,
    Falling,
    Counting,
}

struct GameState {
    horizontal_velocity: Fixed,
    left_pressed: bool,
    right_pressed: bool,
}

fn spawn_pegs(pegs: &mut Pegs, rng: &mut RandomNumberGenerator) {
    let peg_count = 20;
    let screen_width = 140;
    let screen_height = 120;
    let min_x = 20;
    let min_y = 30;

    for _ in 0..peg_count {
        let x = min_x + (rng.next_i32().abs() % (screen_width - min_x));
        let y = min_y + (rng.next_i32().abs() % (screen_height - min_y));

        let force_radius_index =
            (rng.next_i32().abs() % peg::FORCE_RADII.len() as i32) as usize;
        let force_radius =
            Fixed::new(peg::FORCE_RADII[force_radius_index] as i32);

        pegs.add_peg(vec2(Fixed::new(x), Fixed::new(y)), force_radius);
    }
}

fn update_aiming(
    input: &mut ButtonController,
    game_state: &mut GameState,
    ball: &mut Ball,
) -> Result<State, Error> {
    let delta_time = num!(DELTA_TIME);

    let left_currently_pressed = input.is_pressed(Button::LEFT);
    let right_currently_pressed = input.is_pressed(Button::RIGHT);

    if left_currently_pressed && !right_currently_pressed {
        game_state.horizontal_velocity = (game_state.horizontal_velocity
            - num!(VELOCITY_CHANGE_RATE) * delta_time)
            .max(num!(-MAX_HORIZONTAL_VELOCITY));
    } else if right_currently_pressed && !left_currently_pressed {
        game_state.horizontal_velocity = (game_state.horizontal_velocity
            + num!(VELOCITY_CHANGE_RATE) * delta_time)
            .min(num!(MAX_HORIZONTAL_VELOCITY));
    }

    game_state.left_pressed = left_currently_pressed;
    game_state.right_pressed = right_currently_pressed;

    if input.is_just_pressed(Button::A) {
        ball.velocity =
            vec2(game_state.horizontal_velocity, num!(BALL_START_Y));
        return Ok(State::Falling);
    }

    Ok(State::Aiming)
}

fn update_falling(
    ball: &mut Ball,
    pegs: &mut Pegs,
    physics: &mut PhysicsState,
) -> Result<State, Error> {
    crate::bench::start("BALL");
    physics::update_ball(ball, pegs, num!(DELTA_TIME), physics);
    crate::bench::stop("BALL");

    if ball.position.y > num!(SCREEN_BOTTOM) {
        return Ok(State::Counting);
    }
    Ok(State::Falling)
}

fn update_counting(ball: &mut Ball, pegs: &mut Pegs) -> Result<State, Error> {
    ball.position = vec2(num!(BALL_START_X), num!(BALL_START_Y));
    for i in 0..pegs.count {
        if pegs.is_touched(i) {
            pegs.set_present(i, false);
        }
    }
    Ok(State::Aiming)
}

pub fn main(gba: &mut agb::Gba) -> Result<Scene, Error> {
    let mut state = State::Aiming;
    let mut game_state = GameState {
        horizontal_velocity: num!(BALL_START_Y), // Start with no horizontal velocity
        left_pressed: false,
        right_pressed: false,
    };

    let mut gfx = gba.graphics.get();
    let mut input = ButtonController::new();
    let mut timers = gba.timers.timers();

    let mut ball = Ball::new(vec2(num!(BALL_START_X), num!(BALL_START_Y)));
    let mut pegs = Pegs::new();
    let mut rng = RandomNumberGenerator::new();

    spawn_pegs(&mut pegs, &mut rng);

    let mut physics = physics::PhysicsState::new(&pegs);

    crate::bench::init(&mut timers);

    loop {
        input.update();

        state = match state {
            State::Aiming => {
                update_aiming(&mut input, &mut game_state, &mut ball)?
            }
            State::Falling => {
                crate::bench::start("FALLING");
                let state = update_falling(&mut ball, &mut pegs, &mut physics)?;
                crate::bench::stop("FALLING");
                state
            }
            State::Counting => {
                crate::bench::log();
                update_counting(&mut ball, &mut pegs)?
            }
        };

        let mut frame = gfx.frame();
        for i in 0..pegs.count {
            if pegs.is_present(i) {
                pegs.show(i, &mut frame);
            }
        }
        ball.show(&mut frame);
        frame.commit();
    }
}
