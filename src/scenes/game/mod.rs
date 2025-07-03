use crate::Fixed;
use crate::error::Error;
use crate::physics::Physics;
use crate::scenes::Scene;
use crate::scenes::game::bucket::Bucket;
use crate::scenes::game::score::Score;
use agb::fixnum::{num, vec2};
use agb::input::{Button, ButtonController};
use agb::rng::RandomNumberGenerator;
use ball::Ball;
use peg::{Kind, Pegs};

pub mod ball;
pub mod bucket;
pub mod peg;
pub mod score;

const MAX_HORIZONTAL_VELOCITY: f32 = 100.0;
const VELOCITY_CHANGE_RATE: f32 = 120.0;
const DELTA_TIME: f32 = 1.0 / 60.0;
const BALL_START_X: f32 = 21.0;
const BALL_START_Y: f32 = 0.0;
const BUCKET_START_X: f32 = 80.0;
const BUCKET_START_Y: f32 = 140.0;
const SCREEN_BOTTOM: f32 = 168.0;
const MAX_PEGS: usize = 50;

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

fn spawn_pegs(rng: &mut RandomNumberGenerator) -> Pegs<MAX_PEGS> {
    let peg_count = 50;
    let screen_width = 140;
    let screen_height = 120;
    let min_x = 20;
    let min_y = 30;

    let mut positions = [vec2(num!(0), num!(0)); MAX_PEGS];
    let mut force_radius_squared = [num!(20); MAX_PEGS];
    let mut showable = [false; MAX_PEGS];
    let mut collidable = [false; MAX_PEGS];
    let mut kind = [Kind::Blue; MAX_PEGS];

    for i in 0..peg_count {
        let x = min_x + (rng.next_i32().abs() % (screen_width - min_x));
        let y = min_y + (rng.next_i32().abs() % (screen_height - min_y));

        let force_radius_index =
            (rng.next_i32().abs() % peg::FORCE_RADII.len() as i32) as usize;
        let force_radius =
            Fixed::new(peg::FORCE_RADII[force_radius_index] as i32);

        positions[i] = vec2(Fixed::new(x), Fixed::new(y));
        force_radius_squared[i] = force_radius * force_radius;
        showable[i] = true;
        collidable[i] = true;

        kind[i] = if rng.next_i32() > 0 {
            Kind::Blue
        } else {
            Kind::Red
        }
    }

    Pegs::new(positions, force_radius_squared, showable, collidable, kind)
}

fn update_aiming(
    input: &mut ButtonController,
    game_state: &mut GameState,
    ball: &mut Ball,
) -> Result<State, Error> {
    ball.reset_sprite();

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
    pegs: &mut Pegs<MAX_PEGS>,
    physics: &mut Physics<MAX_PEGS>,
    score: &mut Score,
    bucket: &Bucket,
) -> Result<State, Error> {
    ball.update();

    crate::bench::start("UPDATE_BALL_TOP");
    let (position, velocity, touched) = physics
        .move_and_collide::<{ ball::RADIUS }, { peg::RADIUS }, 200, 0, 0, 160, 180>(
            ball.position,
            ball.velocity,
            &pegs.positions,
            &pegs.collidable,
            num!(DELTA_TIME),
            &bucket.walls,
        )?;
    ball.position = position;
    ball.velocity = velocity;
    crate::bench::stop("UPDATE_BALL_TOP");

    for &i in touched {
        pegs.collidable[i] = false;

        match pegs.kind[i] {
            Kind::Blue => score.add_base(1),
            Kind::Red => score.add_mult(1),
        }
    }

    if ball.position.y > num!(SCREEN_BOTTOM) {
        return Ok(State::Counting);
    }

    if bucket.is_in_bucket(ball.position) {
        agb::println!("Bucketed!");
    }

    Ok(State::Falling)
}

fn update_counting(
    ball: &mut Ball,
    pegs: &mut Pegs<MAX_PEGS>,
    score: &mut Score,
) -> Result<State, Error> {
    ball.position = vec2(num!(BALL_START_X), num!(BALL_START_Y));
    for i in 0..MAX_PEGS {
        if !pegs.collidable[i] {
            pegs.showable[i] = false;
        }
    }

    score.commit();

    Ok(State::Aiming)
}

pub fn main(gba: &mut agb::Gba) -> Result<Scene, Error> {
    let mut state = State::Aiming;
    let mut game_state = GameState {
        horizontal_velocity: num!(BALL_START_Y),
        left_pressed: false,
        right_pressed: false,
    };

    let mut gfx = gba.graphics.get();
    let mut input = ButtonController::new();
    let mut timers = gba.timers.timers();

    let mut ball = Ball::new(vec2(num!(BALL_START_X), num!(BALL_START_Y)));
    let mut bucket =
        Bucket::new(vec2(num!(BUCKET_START_X), num!(BUCKET_START_Y)));
    let mut rng = RandomNumberGenerator::new();
    let mut pegs = spawn_pegs(&mut rng);
    let mut physics =
        Physics::<MAX_PEGS>::new(&pegs.positions, &pegs.collidable)?;
    let mut score = Score::new();

    crate::bench::init(&mut timers);

    loop {
        input.update();

        crate::bench::start("PEG_UPDATE");
        physics.move_from_fields::<3000, 10, 10, 10, 150, 110, 10>(
            &mut pegs.positions,
            &mut pegs.velocities,
            &pegs.collidable,
            &pegs.force_radius_squared,
            num!(DELTA_TIME),
        )?;
        crate::bench::stop("PEG_UPDATE");

        bucket.update();

        state = match state {
            State::Aiming => {
                update_aiming(&mut input, &mut game_state, &mut ball)?
            }
            State::Falling => update_falling(
                &mut ball,
                &mut pegs,
                &mut physics,
                &mut score,
                &bucket,
            )?,
            State::Counting => {
                crate::bench::log();
                update_counting(&mut ball, &mut pegs, &mut score)?
            }
        };

        let mut frame = gfx.frame();

        pegs.show(&mut frame);
        ball.show(&mut frame);
        bucket.show(&mut frame);

        frame.commit();
    }
}
