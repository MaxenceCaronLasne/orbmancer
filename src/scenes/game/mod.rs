use crate::error::Error;
use crate::scenes::Scene;
use crate::types::Fixed;
use agb::fixnum::{num, vec2};
use agb::input::{Button, ButtonController};
use agb::rng::RandomNumberGenerator;
use alloc::alloc::Global;
use alloc::vec::Vec;
use ball::Ball;
use peg::Peg;

pub mod ball;
pub mod peg;
pub mod peg_physics;
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

fn spawn_pegs(pegs: &mut Vec<Peg>, rng: &mut RandomNumberGenerator) {
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

        pegs.push(Peg::new(vec2(Fixed::new(x), Fixed::new(y)), force_radius));
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

    if game_state.left_pressed && !left_currently_pressed {
        agb::println!(
            "Horizontal velocity: {}",
            game_state.horizontal_velocity
        );
    }
    if game_state.right_pressed && !right_currently_pressed {
        agb::println!(
            "Horizontal velocity: {}",
            game_state.horizontal_velocity
        );
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

fn update_falling(ball: &mut Ball, pegs: &mut [Peg]) -> Result<State, Error> {
    physics::move_and_collide(ball, pegs, num!(DELTA_TIME));

    if ball.position.y > num!(SCREEN_BOTTOM) {
        return Ok(State::Counting);
    }

    Ok(State::Falling)
}

fn update_counting(
    ball: &mut Ball,
    pegs: &mut Vec<Peg>,
) -> Result<State, Error> {
    ball.position = vec2(num!(BALL_START_X), num!(BALL_START_Y));
    pegs.retain(|p| !p.is_touched());
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

    let mut ball = Ball::new(vec2(num!(BALL_START_X), num!(BALL_START_Y)));
    let mut pegs = Vec::<Peg>::new_in(Global);
    let mut rng = RandomNumberGenerator::new();

    spawn_pegs(&mut pegs, &mut rng);

    let mut peg_physics_frame_counter = 0u8;

    loop {
        input.update();

        peg_physics_frame_counter = peg_physics_frame_counter.wrapping_add(1);
        peg_physics::update_peg_forces(
            &mut pegs,
            num!(DELTA_TIME), //* num!(4.0),
        );
        if peg_physics_frame_counter % 4 == 0 {}

        state = match state {
            State::Aiming => {
                update_aiming(&mut input, &mut game_state, &mut ball)?
            }
            State::Falling => update_falling(&mut ball, &mut pegs)?,
            State::Counting => update_counting(&mut ball, &mut pegs)?,
        };

        //busy_wait_for_vblank();
        let mut frame = gfx.frame();
        for p in pegs.iter_mut() {
            p.show(&mut frame);
        }
        ball.show(&mut frame);
        frame.commit();
    }
}
