use crate::Fixed;
use crate::error::Error;
use crate::physics::Physics;
use crate::scenes::Scene;
use crate::scenes::game::bucket::Bucket;
use crate::scenes::game::score::Score;
use agb::display::GraphicsFrame;
use agb::fixnum::{num, vec2};
use agb::input::{Button, ButtonController};
use agb::rng::RandomNumberGenerator;
use alloc::vec;
use alloc::vec::Vec;
use ball::Ball;
use direction_viewer::DirectionViewer;
use effect::{BallData, BucketEffect};
use peg::{Kind, Pegs};

pub mod ball;
pub mod bucket;
pub mod direction_viewer;
pub mod effect;
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
const TARGET_SCORE: i32 = 1000;

enum State {
    Aiming,
    Falling,
    Counting { is_bucketed: bool },
}

struct GameState<const MAX_PEGS: usize> {
    ball: Ball,
    ball_data: Vec<BallData>,
    bucket: Bucket,
    bucket_effects: Vec<BucketEffect>,
    current_ball_data: Option<BallData>,
    horizontal_velocity: Fixed,
    pegs: Pegs<MAX_PEGS>,
    physics: Physics<MAX_PEGS>,
    score: Score,
    state: State,
    direction_viewer: DirectionViewer,
}

impl<const MAX_PEGS: usize> GameState<MAX_PEGS> {
    pub fn new() -> Result<Self, Error> {
        let rng = &mut RandomNumberGenerator::new();
        let pegs = Self::spawn_pegs(rng);
        let physics =
            Physics::<MAX_PEGS>::new(&pegs.positions, &pegs.collidable)?;

        Ok(Self {
            ball: Ball::new(vec2(num!(BALL_START_X), num!(BALL_START_Y))),
            ball_data: vec![
                BallData::empty(),
                BallData::empty(),
                BallData::empty(),
            ],
            bucket: Bucket::new(vec2(
                num!(BUCKET_START_X),
                num!(BUCKET_START_Y),
            )),
            bucket_effects: vec![BucketEffect::Identity],
            current_ball_data: None,
            direction_viewer: DirectionViewer::new(vec2(
                num!(BALL_START_X),
                num!(BALL_START_Y),
            )),
            horizontal_velocity: num!(BALL_START_Y),
            pegs,
            physics,
            score: Score::new(),
            state: State::Aiming,
        })
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

    pub fn pop_ball(&mut self) -> Result<(), Error> {
        if let Some(ball_data) = self.ball_data.pop() {
            self.current_ball_data = Some(ball_data);
            Ok(())
        } else {
            Err(Error::NoBalls)
        }
    }

    pub fn update(
        &mut self,
        input: &mut ButtonController,
    ) -> Result<Scene, Error> {
        input.update();

        crate::bench::start("PEG_UPDATE");
        self.physics
            .move_from_fields::<3000, 10, 10, 10, 150, 110, 15>(
                &mut self.pegs.positions,
                &mut self.pegs.velocities,
                &self.pegs.collidable,
                &self.pegs.force_radius_squared,
                num!(DELTA_TIME),
            )?;

        crate::bench::stop("PEG_UPDATE");

        self.bucket.update();

        self.state = match self.state {
            State::Aiming => self.update_aiming(input)?,
            State::Falling => self.update_falling()?,
            State::Counting { is_bucketed } => {
                crate::bench::log();
                let res = self.update_counting(is_bucketed)?;

                if self.is_winning() {
                    return Ok(Scene::Win);
                }

                match self.pop_ball() {
                    Ok(_) => {}
                    Err(Error::NoBalls) => {
                        return Ok(Scene::GameOver);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }

                res
            }
        };

        Ok(Scene::Game)
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame) {
        self.pegs.show(frame);
        self.ball.show(frame);
        self.bucket.show(frame);
        if matches!(self.state, State::Aiming) {
            self.direction_viewer.show(frame);
        }
    }

    fn update_aiming(
        &mut self,
        input: &ButtonController,
    ) -> Result<State, Error> {
        self.ball.reset_sprite();

        let delta_time = num!(DELTA_TIME);

        let left_currently_pressed = input.is_pressed(Button::LEFT);
        let right_currently_pressed = input.is_pressed(Button::RIGHT);

        if left_currently_pressed && !right_currently_pressed {
            self.horizontal_velocity = (self.horizontal_velocity
                - num!(VELOCITY_CHANGE_RATE) * delta_time)
                .max(num!(-MAX_HORIZONTAL_VELOCITY));
        } else if right_currently_pressed && !left_currently_pressed {
            self.horizontal_velocity = (self.horizontal_velocity
                + num!(VELOCITY_CHANGE_RATE) * delta_time)
                .min(num!(MAX_HORIZONTAL_VELOCITY));
        }

        self.direction_viewer
            .update_direction(self.horizontal_velocity);

        if input.is_just_pressed(Button::A) {
            self.ball.velocity =
                vec2(self.horizontal_velocity, num!(BALL_START_Y));
            return Ok(State::Falling);
        }

        Ok(State::Aiming)
    }

    fn update_falling(&mut self) -> Result<State, Error> {
        self.ball.update();

        crate::bench::start("UPDATE_BALL_TOP");
        let (position, velocity, touched) = self.physics
        .move_and_collide::<{ ball::RADIUS }, { peg::RADIUS }, 200, 0, 0, 160, 180>(
            self.ball.position,
            self.ball.velocity,
            &self.pegs.positions,
            &self.pegs.collidable,
            num!(DELTA_TIME),
            &self.bucket.walls,
        )?;
        self.ball.position = position;
        self.ball.velocity = velocity;
        crate::bench::stop("UPDATE_BALL_TOP");

        for &i in touched {
            self.pegs.collidable[i] = false;

            let (mut base, mut mult) = match self.pegs.kind[i] {
                Kind::Blue => (1, 0),
                Kind::Red => (0, 1),
            };

            for pe in &self.ball_data {
                (base, mult) = pe.passive().apply(base, mult);
            }

            if let Some(ball_data) = &self.current_ball_data {
                (base, mult) = ball_data.active().apply(base, mult);
            }

            self.score.base += base;
            self.score.mult += mult;

            agb::println!("Added: ({}, {})", base, mult);
        }

        if self.ball.position.y > num!(SCREEN_BOTTOM) {
            return Ok(State::Counting { is_bucketed: false });
        }

        if self.bucket.is_in_bucket(self.ball.position) {
            return Ok(State::Counting { is_bucketed: true });
        }

        Ok(State::Falling)
    }

    fn update_counting(&mut self, is_bucketed: bool) -> Result<State, Error> {
        self.ball.position = vec2(num!(BALL_START_X), num!(BALL_START_Y));

        for i in 0..MAX_PEGS {
            if !self.pegs.collidable[i] {
                self.pegs.showable[i] = false;
            }
        }

        if is_bucketed {
            agb::println!("Bucket!");
            for e in &self.bucket_effects {
                e.apply(&mut self.score);
            }
        }

        self.score.commit();

        Ok(State::Aiming)
    }

    pub fn is_winning(&self) -> bool {
        self.score.total() > TARGET_SCORE
    }
}

pub fn main(gba: &mut agb::Gba) -> Result<Scene, Error> {
    let mut gfx = gba.graphics.get();
    let mut input = ButtonController::new();
    let mut timers = gba.timers.timers();

    let mut game_state = GameState::<50>::new()?;
    game_state.pop_ball()?;

    crate::bench::init(&mut timers);

    loop {
        match game_state.update(&mut input) {
            Ok(Scene::Game) => {}
            res => return res,
        }

        let mut frame = gfx.frame();

        game_state.show(&mut frame);

        frame.commit();
    }
}
