use crate::{
    error::Error,
    physics::Physics,
    save::Save,
    scenes::{Scene, game::bucket::Bucket},
};
use agb::{
    InternalAllocator,
    display::{GraphicsFrame, font::AlignmentKind, tiled::RegularBackground},
    fixnum::{num, vec2},
    input::{Button, ButtonController},
    rng::RandomNumberGenerator,
};
use alloc::{boxed::Box, vec, vec::Vec};
use ball::Ball;
use config::GameConfig;
use counter::Counter;
use effect::{BallData, BucketEffect};
use inventory::InventoryPresenter;
use jauge::Jauge;
use launcher::Launcher;
use peg::Pegs;
use score::ScoreManager;
use shake::{ScreenShake, WhiteFlash};
use text_box::TextBox;

mod background;
mod ball;
mod bucket;
mod config;
mod counter;
mod effect;
mod inventory;
mod jauge;
mod launcher;
mod peg;
mod score;
mod shake;
mod text_box;

#[cfg(test)]
mod test;

type InventoryIndex = usize;

struct StateManager {
    current: State,
    previous: Option<State>,
}

impl StateManager {
    fn new() -> Self {
        Self {
            current: State::Aiming,
            previous: None,
        }
    }

    fn current(&self) -> State {
        self.current
    }

    fn previous(&self) -> Option<State> {
        self.previous
    }

    fn transition_to(&mut self, new_state: State) {
        if new_state != self.current {
            self.previous = Some(self.current);
        }
        self.current = new_state;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Aiming,
    Falling,
    InInventory,
    Counting { is_bucketed: bool },
}

struct GameState<const MAX_PEGS: usize> {
    ball: Ball,
    inventory: Vec<BallData>,
    bucket: Bucket,
    bucket_effects: Vec<BucketEffect>,
    current_ball_data: Option<BallData>,
    pegs: Box<Pegs<MAX_PEGS>, InternalAllocator>,
    physics: Box<Physics<MAX_PEGS>, InternalAllocator>,
    score_manager: ScoreManager,
    state_manager: StateManager,
    background: RegularBackground,
    white_background: RegularBackground,
    mult_counter: Counter,
    base_counter: Counter,
    coin_counter: Counter,
    inventory_presenter: InventoryPresenter,
    selected_inventory_index: InventoryIndex,
    text_box: TextBox,
    launcher: Launcher,
    jauge: Jauge<0, 50>,
    screen_shake: ScreenShake,
    white_flash: WhiteFlash,
    rng: RandomNumberGenerator,
}

impl<const MAX_PEGS: usize> GameState<MAX_PEGS> {
    pub fn new(save: &Save) -> Result<Self, Error> {
        let mut rng = RandomNumberGenerator::new();
        let pegs = Box::new_in(
            Pegs::<MAX_PEGS>::spawn_pegs::<
                { GameConfig::WALL_LEFT },
                { GameConfig::WALL_RIGHT },
            >(&mut rng),
            InternalAllocator,
        );
        let physics = Box::new_in(
            Physics::<MAX_PEGS>::new(&pegs.positions, &pegs.collidable)?,
            InternalAllocator,
        );

        Ok(Self {
            ball: Ball::new(GameConfig::ball_start_pos()),
            inventory: effect::from_kinds(save.inventory()),
            bucket: Bucket::new(GameConfig::bucket_start_pos()),
            bucket_effects: vec![BucketEffect::Identity],
            current_ball_data: None,
            launcher: Launcher::new(GameConfig::ball_start_pos()),
            pegs,
            physics,
            score_manager: ScoreManager::new(save.coins()),
            state_manager: StateManager::new(),
            background: background::new(),
            white_background: background::new_white(),
            base_counter: Counter::new(
                vec2(num!(217), num!(125)),
                AlignmentKind::Left,
            ),
            mult_counter: Counter::new(
                vec2(num!(206), num!(125)),
                AlignmentKind::Right,
            ),
            coin_counter: Counter::new(
                vec2(num!(234), num!(145)),
                AlignmentKind::Right,
            ),
            inventory_presenter: InventoryPresenter::new(vec2(
                num!(8),
                num!(16),
            )),
            selected_inventory_index: 0,
            text_box: TextBox::new(vec2(189, 5), 46),
            jauge: Jauge::new(vec2(num!(184), num!(104))),
            screen_shake: ScreenShake::inactive(),
            white_flash: WhiteFlash::new(),
            rng,
        })
    }

    pub fn pop_ball(&mut self) -> Result<(), Error> {
        if let Some(ball_data) = self.inventory.pop() {
            self.current_ball_data = Some(ball_data);
            Ok(())
        } else {
            Err(Error::NoBalls)
        }
    }

    fn update_pegs(&mut self) -> Result<(), Error> {
        crate::bench::start("PEG_UPDATE");
        let result = self.physics
            .move_from_fields::<3000, 10, { GameConfig::WALL_LEFT }, 10, { GameConfig::WALL_RIGHT }, 110, 15>(
                &mut self.pegs.positions,
                &mut self.pegs.velocities,
                &self.pegs.collidable,
                &self.pegs.force_radius_squared,
                num!(GameConfig::DELTA_TIME),
            );
        crate::bench::stop("PEG_UPDATE");
        result
    }

    fn update_bucket(&mut self) {
        self.bucket
            .update::<{ GameConfig::WALL_LEFT }, { GameConfig::WALL_RIGHT }>();
    }

    pub fn update(
        &mut self,
        input: &mut ButtonController,
    ) -> Result<Scene, Error> {
        input.update();
        self.text_box.update();
        self.screen_shake.update(&mut self.rng);
        self.white_flash.update();

        let new_state = match self.state_manager.current() {
            State::Aiming => self.update_aiming(input)?,
            State::Falling => self.update_falling(input)?,
            State::InInventory => self.update_inventory(input)?,
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

                self.set_text_box_to_current_data();
                res
            }
        };

        self.state_manager.transition_to(new_state);

        Ok(Scene::Game)
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame) {
        if self.screen_shake.is_active() {
            self.background
                .set_scroll_pos(self.screen_shake.offset().round());
        } else {
            self.background.set_scroll_pos((0, 0));
        }

        if self.white_flash.is_active() {
            self.white_background.show(frame);
        } else {
            self.pegs.show(frame);
            self.ball.show(frame);
            self.bucket.show(frame);
            self.background.show(frame);
            self.base_counter.show(frame);
            self.mult_counter.show(frame);
            self.coin_counter.show(frame);
            self.inventory_presenter.show(frame, &self.inventory);
            self.text_box.show(frame);
            self.jauge.show(frame);

            if matches!(self.state_manager.current(), State::Aiming) {
                self.launcher.show(frame);
            }
        }
    }

    fn update_aiming(
        &mut self,
        input: &ButtonController,
    ) -> Result<State, Error> {
        self.ball.reset_sprite();

        self.update_pegs()?;
        self.update_bucket();

        let delta = num!(GameConfig::DELTA_TIME);

        let is_left_pressed = input.is_pressed(Button::LEFT);
        let is_right_pressed = input.is_pressed(Button::RIGHT);

        if is_left_pressed && !is_right_pressed {
            self.launcher.turn_left(delta);
        } else if is_right_pressed && !is_left_pressed {
            self.launcher.turn_right(delta);
        }

        if input.is_just_pressed(Button::A) {
            self.ball.velocity =
                vec2(self.launcher.velocity(), num!(GameConfig::BALL_START_Y));

            return Ok(State::Falling);
        }

        if input.is_just_pressed(Button::SELECT) {
            if let Some(ball_data) = self.inventory.first() {
                self.text_box.set_text(ball_data.kind().description());
            }
            return Ok(State::InInventory);
        }

        Ok(State::Aiming)
    }

    fn set_text_box_to_current_data(&mut self) {
        match self.current_ball_data {
            Some(ball_data) => {
                self.text_box.set_text(ball_data.kind().description())
            }
            None => self.text_box.remove(),
        }
    }

    fn update_inventory(
        &mut self,
        input: &ButtonController,
    ) -> Result<State, Error> {
        if input.is_just_pressed(Button::SELECT) {
            self.set_text_box_to_current_data();
            if let Some(last_state) = self.state_manager.previous() {
                return Ok(last_state);
            } else {
                return Err(Error::NoLastState);
            }
        }

        if input.is_just_pressed(Button::UP)
            && self.selected_inventory_index > 0
        {
            self.selected_inventory_index -= 1;
            self.text_box.set_text(
                self.inventory[self.selected_inventory_index]
                    .kind()
                    .description(),
            );
        }

        if input.is_just_pressed(Button::DOWN)
            && self.selected_inventory_index < self.inventory.len() - 1
        {
            self.selected_inventory_index += 1;
            self.text_box.set_text(
                self.inventory[self.selected_inventory_index]
                    .kind()
                    .description(),
            );
        }

        self.inventory_presenter.update();

        Ok(State::InInventory)
    }

    fn update_falling(
        &mut self,
        input: &ButtonController,
    ) -> Result<State, Error> {
        self.ball.update();
        self.update_pegs()?;
        self.update_bucket();

        crate::bench::start("UPDATE_BALL_TOP");
        let (position, velocity, touched) = self.physics
        .move_and_collide::<{ ball::RADIUS }, { peg::RADIUS }, 200, { GameConfig::WALL_LEFT }, 0, { GameConfig::WALL_RIGHT }, 180>(
            self.ball.position,
            self.ball.velocity,
            &self.pegs.positions,
            &self.pegs.collidable,
            num!(GameConfig::DELTA_TIME),
            &self.bucket.walls,
        )?;
        self.ball.position = position;
        self.ball.velocity = velocity;
        crate::bench::stop("UPDATE_BALL_TOP");

        for &i in touched {
            self.pegs.collidable[i] = false;
            self.pegs.showable[i] = false;

            self.score_manager.process_peg_hit(
                self.pegs.kind[i],
                &self.inventory,
                &self.current_ball_data,
                &mut self.mult_counter,
                &mut self.base_counter,
                &mut self.coin_counter,
            );
        }

        if !touched.is_empty() {
            self.screen_shake.start(
                GameConfig::SHAKE_DURATION,
                GameConfig::SHAKE_INTENSITY - 2,
            );
            //self.white_flash.start(GameConfig::FLASH_DURATION);
        }

        if self.ball.position.y > num!(GameConfig::SCREEN_BOTTOM) {
            self.screen_shake
                .start(GameConfig::SHAKE_DURATION, GameConfig::SHAKE_INTENSITY);
            self.white_flash.start(GameConfig::FLASH_DURATION);
            return Ok(State::Counting { is_bucketed: false });
        }

        if self.bucket.is_in_bucket(self.ball.position) {
            self.screen_shake
                .start(GameConfig::SHAKE_DURATION, GameConfig::SHAKE_INTENSITY);
            self.white_flash.start(GameConfig::FLASH_DURATION);
            return Ok(State::Counting { is_bucketed: true });
        }

        if input.is_just_pressed(Button::SELECT) {
            return Ok(State::InInventory);
        }

        Ok(State::Falling)
    }

    fn update_counting(&mut self, is_bucketed: bool) -> Result<State, Error> {
        self.ball.position = GameConfig::ball_start_pos();
        self.update_pegs()?;
        self.update_bucket();

        self.pegs
            .collidable
            .iter()
            .zip(self.pegs.showable.iter_mut())
            .for_each(|(&is_collidable, showable)| {
                if !is_collidable {
                    *showable = false;
                }
            });

        if is_bucketed {
            self.score_manager.process_bucket_bonus(
                &self.bucket_effects,
                &mut self.mult_counter,
                &mut self.base_counter,
                &mut self.coin_counter,
            );
        }

        let (damages, _) = self.score_manager.extract_final_score();
        self.score_manager
            .reset_counters(&mut self.mult_counter, &mut self.base_counter);

        self.jauge.set(
            GameConfig::TARGET_SCORE - damages,
            0,
            GameConfig::TARGET_SCORE,
        );

        Ok(State::Aiming)
    }

    pub fn is_winning(&self) -> bool {
        self.score_manager.is_winning()
    }
}

pub fn main(gba: &mut agb::Gba, save: &mut Save) -> Result<Scene, Error> {
    let mut gfx = gba.graphics.get();
    let mut input = ButtonController::new();
    let mut timers = gba.timers.timers();

    let mut game_state = GameState::<50>::new(save)?;
    game_state.pop_ball()?;

    crate::bench::init(&mut timers);

    game_state.set_text_box_to_current_data();

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
