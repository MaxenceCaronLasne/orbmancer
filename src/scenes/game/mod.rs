use crate::Fixed;
use crate::error::Error;
use crate::physics::Physics;
use crate::save::Save;
use crate::scenes::Scene;
use crate::scenes::game::bucket::Bucket;
use crate::scenes::game::score::Score;
use agb::InternalAllocator;
use agb::display::GraphicsFrame;
use agb::display::tiled::RegularBackground;
use agb::fixnum::{num, vec2};
use agb::input::{Button, ButtonController};
use agb::rng::RandomNumberGenerator;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use ball::Ball;
use direction_viewer::DirectionViewer;
use effect::{BallData, BucketEffect};
use peg::{Kind, Pegs};
use counter::{Counter, Alignment};

mod background;
pub mod ball;
pub mod bucket;
pub mod direction_viewer;
pub mod effect;
pub mod peg;
pub mod score;
pub mod counter;

#[cfg(test)]
mod test;

const MAX_INPUT_VELOCITY: f32 = 100.0;
const VELOCITY_CHANGE_RATE: f32 = 120.0;
const DELTA_TIME: f32 = 1.0 / 60.0;
const BALL_START_X: f32 = 100.0;
const BALL_START_Y: f32 = 0.0;
const BUCKET_START_X: f32 = 80.0;
const BUCKET_START_Y: f32 = 140.0;
const SCREEN_BOTTOM: f32 = 168.0;
const TARGET_SCORE: i32 = 1000;
const WALL_LEFT: i32 = 3 * 8 + 1;
const WALL_RIGHT: i32 = WALL_LEFT + 160 - 8 - 1;

enum State {
    Aiming,
    Falling,
    Counting { is_bucketed: bool },
}

struct GameState<const MAX_PEGS: usize> {
    ball: Ball,
    inventory: Vec<BallData>,
    bucket: Bucket,
    bucket_effects: Vec<BucketEffect>,
    current_ball_data: Option<BallData>,
    input_velocity: Fixed,
    pegs: Box<Pegs<MAX_PEGS>, InternalAllocator>,
    physics: Box<Physics<MAX_PEGS>, InternalAllocator>,
    current_score: Option<Score>,
    damages: score::Damage,
    coins: score::Coins,
    state: State,
    direction_viewer: DirectionViewer,
    background: RegularBackground,
    mult_counter: Counter,
    base_counter: Counter,
    coin_counter: Counter,
}

impl<const MAX_PEGS: usize> GameState<MAX_PEGS> {
    pub fn new(save: &Save) -> Result<Self, Error> {
        // Get current stack address
        let stack_var = 0u32;
        let stack_addr = &stack_var as *const u32 as usize;

        let rng = &mut RandomNumberGenerator::new();
        let pegs = Box::new_in(Self::spawn_pegs(rng), InternalAllocator);
        let physics = Box::new_in(
            Physics::<MAX_PEGS>::new(&pegs.positions, &pegs.collidable)?,
            InternalAllocator,
        );

        // Print addresses and sizes
        agb::println!("Stack address: 0x{:08X}", stack_addr);
        agb::println!(
            "Pegs address: 0x{:08X}, size: {} bytes",
            pegs.as_ref() as *const _ as usize,
            core::mem::size_of::<Pegs<MAX_PEGS>>()
        );
        agb::println!(
            "Physics address: 0x{:08X}, size: {} bytes",
            physics.as_ref() as *const _ as usize,
            core::mem::size_of::<Physics<MAX_PEGS>>()
        );

        Ok(Self {
            ball: Ball::new(vec2(num!(BALL_START_X), num!(BALL_START_Y))),
            inventory: effect::from_kinds(save.inventory()),
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
            input_velocity: num!(BALL_START_Y),
            pegs,
            physics,
            current_score: None,
            damages: 0,
            coins: save.coins(),
            state: State::Aiming,
            background: background::new(),
            base_counter: Counter::new(vec2(num!(206), num!(125)), Alignment::RightToLeft),
            mult_counter: Counter::new(vec2(num!(217), num!(125)), Alignment::LeftToRight),
            coin_counter: Counter::new(vec2(num!(234), num!(145)), Alignment::RightToLeft),
        })
    }

    fn spawn_pegs(rng: &mut RandomNumberGenerator) -> Pegs<MAX_PEGS> {
        let peg_count = 50;
        let screen_height = 120;
        let min_y = 30;

        let mut positions = [vec2(num!(0), num!(0)); MAX_PEGS];
        let mut force_radius_squared = [num!(20); MAX_PEGS];
        let mut showable = [false; MAX_PEGS];
        let mut collidable = [false; MAX_PEGS];
        let mut kind = [Kind::Blue; MAX_PEGS];

        for i in 0..peg_count {
            let x = WALL_LEFT + (rng.next_i32().abs() % (WALL_RIGHT - WALL_LEFT));
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
            } else if rng.next_i32() > 0 {
                Kind::Yellow
            } else {
                Kind::Red
            }
        }

        Pegs::new(positions, force_radius_squared, showable, collidable, kind)
    }

    pub fn pop_ball(&mut self) -> Result<(), Error> {
        if let Some(ball_data) = self.inventory.pop() {
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
            .move_from_fields::<3000, 10, WALL_LEFT, 10, WALL_RIGHT, 110, 15>(
                &mut self.pegs.positions,
                &mut self.pegs.velocities,
                &self.pegs.collidable,
                &self.pegs.force_radius_squared,
                num!(DELTA_TIME),
            )?;
        crate::bench::stop("PEG_UPDATE");

        self.bucket.update::<WALL_LEFT, WALL_RIGHT>();

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
        self.background.show(frame);
        self.base_counter.show(frame);
        self.mult_counter.show(frame);
        self.coin_counter.show(frame);

        if matches!(self.state, State::Aiming) {
            self.direction_viewer.show(frame);
        }
    }

    fn update_input_velocity_to_left(&mut self, delta: Fixed) {
        self.input_velocity -= num!(VELOCITY_CHANGE_RATE) * delta;
        self.input_velocity = self
            .input_velocity
            .clamp(num!(-MAX_INPUT_VELOCITY), num!(MAX_INPUT_VELOCITY));
    }

    fn update_input_velocity_to_right(&mut self, delta: Fixed) {
        self.input_velocity += num!(VELOCITY_CHANGE_RATE) * delta;
        self.input_velocity = self
            .input_velocity
            .clamp(num!(-MAX_INPUT_VELOCITY), num!(MAX_INPUT_VELOCITY));
    }

    fn update_aiming(
        &mut self,
        input: &ButtonController,
    ) -> Result<State, Error> {
        self.ball.reset_sprite();

        let delta = num!(DELTA_TIME);

        let is_left_pressed = input.is_pressed(Button::LEFT);
        let is_right_pressed = input.is_pressed(Button::RIGHT);

        if is_left_pressed && !is_right_pressed {
            self.update_input_velocity_to_left(delta);
        } else if is_right_pressed && !is_left_pressed {
            self.update_input_velocity_to_right(delta);
        }

        self.direction_viewer.update_direction(self.input_velocity);

        if input.is_just_pressed(Button::A) {
            self.ball.velocity = vec2(self.input_velocity, num!(BALL_START_Y));
            return Ok(State::Falling);
        }

        Ok(State::Aiming)
    }

    fn update_falling(&mut self) -> Result<State, Error> {
        self.ball.update();

        crate::bench::start("UPDATE_BALL_TOP");
        let (position, velocity, touched) = self.physics
        .move_and_collide::<{ ball::RADIUS }, { peg::RADIUS }, 200, WALL_LEFT, 0, WALL_RIGHT, 180>(
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

            let mut score = self.current_score.unwrap_or(Score::new(0, 0, 0));

            score = score.apply(match self.pegs.kind[i] {
                Kind::Blue => Score::new(1, 0, 0),
                Kind::Red => Score::new(0, 1, 0),
                Kind::Yellow => Score::new(0, 0, 1),
            });

            for pe in &self.inventory {
                score = pe.passive().apply(score);
            }

            if let Some(ball_data) = &self.current_ball_data {
                score = ball_data.active().apply(score);
            }

            self.mult_counter.set(score.mult);
            self.base_counter.set(score.base);
            self.coin_counter.set(self.coins + score.coins);
            self.current_score = Some(score);
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
            let mut score = self.current_score.unwrap_or(Score::new(0, 1, 0));

            agb::println!("Bucket!");
            for e in &self.bucket_effects {
                score = e.apply(score);
            }

            self.mult_counter.set(score.mult);
            self.base_counter.set(score.base);
            self.coin_counter.set(self.coins + score.coins);
            self.current_score = Some(score);
        }

        if let Some(score) = self.current_score {
            let (damages, coins) = score.extract();
            self.damages += damages;
            self.coins += coins;
        }

        self.current_score = None;
        self.mult_counter.reset();
        self.base_counter.reset();

        agb::println!("Score: {} damages, {} coins", self.damages, self.coins);

        Ok(State::Aiming)
    }

    pub fn is_winning(&self) -> bool {
        self.damages > TARGET_SCORE
    }
}

pub fn main(gba: &mut agb::Gba, save: &mut Save) -> Result<Scene, Error> {
    let mut gfx = gba.graphics.get();
    let mut input = ButtonController::new();
    let mut timers = gba.timers.timers();

    let mut game_state = GameState::<50>::new(save)?;
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
