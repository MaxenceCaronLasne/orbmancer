use super::{
    background::Background,
    ball::Ball,
    config::GameConfig,
    counter::Counter,
    effect::{BallData, BucketEffect},
    input::{InputHandler, InventoryNavigation, MovementDirection},
    inventory::InventoryPresenter,
    jauge::Jauge,
    launcher::Launcher,
    peg::Pegs,
    peg_generator::PegGenerator,
    physics_handler::PhysicsHandler,
    points_pres::PointsPres,
    score::ScoreManager,
    shake::{ScreenShake, WhiteFlash},
    state::{State, StateManager},
    text_box::TextBox,
};
use crate::{
    error::Error, physics::Physics, save::Save, scenes::game::bucket::Bucket,
};
use agb::{
    InternalAllocator,
    display::font::AlignmentKind,
    fixnum::{num, vec2},
    input::ButtonController,
    rng::RandomNumberGenerator,
};
use alloc::{boxed::Box, vec, vec::Vec};
use const_random::const_random;

type InventoryIndex = usize;

pub struct GameState<const MAX_PEGS: usize> {
    // Game data
    inventory: Vec<BallData>,
    current_ball_data: Option<BallData>,
    bucket_effects: Vec<BucketEffect>,
    selected_inventory_index: InventoryIndex,
    rng: RandomNumberGenerator,
    peg_generators: Vec<PegGenerator>,

    // Core systems
    physics: Box<Physics<MAX_PEGS>, InternalAllocator>,
    pegs: Box<Pegs<MAX_PEGS>, InternalAllocator>,
    score_manager: ScoreManager,
    state_manager: StateManager,

    // Game objects
    ball: Ball,
    bucket: Bucket,
    launcher: Launcher,

    // UI components
    base_counter: Counter,
    mult_counter: Counter,
    coin_counter: Counter,
    inventory_presenter: InventoryPresenter,
    text_box: TextBox,
    jauge: Jauge<0, 50>,
    power_jauge: Jauge<0, 50>,
    point_pres: Vec<PointsPres>,

    // Rendering and effects
    background: Background,
    screen_shake: ScreenShake,
    white_flash: WhiteFlash,
}

impl<const MAX_PEGS: usize> GameState<MAX_PEGS> {
    pub fn new(save: &Save) -> Result<Self, Error> {
        let mut rng = RandomNumberGenerator::new_with_seed([
            const_random!(u32),
            const_random!(u32),
            const_random!(u32),
            const_random!(u32),
        ]);
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
            inventory: super::effect::from_kinds(save.inventory()),
            current_ball_data: None,
            bucket_effects: vec![BucketEffect::Identity],
            selected_inventory_index: 0,
            rng,
            peg_generators: Vec::new(),
            physics,
            pegs,
            score_manager: ScoreManager::new(save.coins()),
            state_manager: StateManager::new(),
            ball: Ball::new(GameConfig::ball_start_pos()),
            bucket: Bucket::new(GameConfig::bucket_start_pos()),
            launcher: Launcher::new(GameConfig::ball_start_pos()),
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
            text_box: TextBox::new(vec2(189, 5), 46),
            jauge: Jauge::new(vec2(num!(184), num!(104))),
            point_pres: Vec::new(),
            power_jauge: Jauge::new(GameConfig::power_gauge_pos()),
            background: Background::new(),
            screen_shake: ScreenShake::inactive(),
            white_flash: WhiteFlash::new(),
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

    pub fn is_winning(&self) -> bool {
        self.score_manager.is_winning()
    }

    pub fn state(&self) -> State {
        self.state_manager.current()
    }

    pub fn set_text_to_current_ball(&mut self) {
        match self.current_ball_data {
            Some(ball_data) => {
                self.text_box.set_text(ball_data.kind().description())
            }
            None => self.text_box.remove(),
        }
    }

    pub fn update_aiming(
        &mut self,
        input: &ButtonController,
    ) -> Result<State, Error> {
        self.ball.reset_sprite();
        PhysicsHandler::update_pegs(&mut self.physics, &mut self.pegs)?;
        self.update_peg_generation()?;
        self.bucket
            .update::<{ GameConfig::WALL_LEFT }, { GameConfig::WALL_RIGHT }>();
        self.update_points_pres();

        let delta = num!(GameConfig::DELTA_TIME);

        match InputHandler::get_movement_direction(input) {
            MovementDirection::Left => self.launcher.turn_left(delta),
            MovementDirection::Right => self.launcher.turn_right(delta),
            MovementDirection::None => {}
        }

        if InputHandler::is_fire_pressed(input) {
            self.launcher.start_charging();
        }

        if InputHandler::is_fire_held(input) {
            self.launcher.charge_power(delta);
        }

        // Update power gauge
        let power_value = self.launcher.get_power_for_jauge();
        self.power_jauge
            .set((power_value * num!(100)).floor(), 20, 300);

        if InputHandler::is_fire_released(input) {
            let power = self.launcher.stop_charging();
            self.ball.velocity = vec2(
                self.launcher.velocity() * power,
                num!(GameConfig::BALL_START_Y) * power,
            );
            return Ok(State::Falling);
        }

        if InputHandler::is_inventory_pressed(input) {
            if let Some(ball_data) = self.inventory.first() {
                self.text_box.set_text(ball_data.kind().description());
            }
            return Ok(State::InInventory);
        }

        Ok(State::Aiming)
    }

    pub fn update_inventory(
        &mut self,
        input: &ButtonController,
    ) -> Result<State, Error> {
        self.update_points_pres();
        if InputHandler::is_inventory_pressed(input) {
            self.set_text_to_current_ball();
            return self.state_manager.return_to_previous();
        }

        match InputHandler::get_inventory_navigation(input) {
            InventoryNavigation::Up if self.selected_inventory_index > 0 => {
                self.selected_inventory_index -= 1;
                self.text_box.set_text(
                    self.inventory[self.selected_inventory_index]
                        .kind()
                        .description(),
                );
            }
            InventoryNavigation::Down
                if self.selected_inventory_index < self.inventory.len() - 1 =>
            {
                self.selected_inventory_index += 1;
                self.text_box.set_text(
                    self.inventory[self.selected_inventory_index]
                        .kind()
                        .description(),
                );
            }
            _ => {}
        }

        self.inventory_presenter.update();
        Ok(State::InInventory)
    }

    pub fn update_falling(
        &mut self,
        input: &ButtonController,
    ) -> Result<State, Error> {
        self.ball.update();
        PhysicsHandler::update_pegs(&mut self.physics, &mut self.pegs)?;
        self.update_peg_generation()?;
        self.bucket
            .update::<{ GameConfig::WALL_LEFT }, { GameConfig::WALL_RIGHT }>();
        self.update_points_pres();

        let (position, velocity, touched) =
            PhysicsHandler::move_ball_and_detect_collisions(
                &mut self.physics,
                self.ball.position,
                self.ball.velocity,
                &self.pegs,
                &self.bucket.walls,
            )?;

        self.ball.position = position;
        self.ball.velocity = velocity;

        self.process_peg_collisions(&touched)?;

        if self.ball.position.y > num!(GameConfig::SCREEN_BOTTOM) {
            self.start_end_effects();
            return Ok(State::Counting { bucketed: None });
        }

        if self.bucket.is_in_bucket(self.ball.position) {
            self.start_end_effects();
            return Ok(State::Counting {
                bucketed: self.current_ball_data,
            });
        }

        if InputHandler::is_inventory_pressed(input) {
            return Ok(State::InInventory);
        }

        Ok(State::Falling)
    }

    pub fn update_counting(
        &mut self,
        bucketed_ball: Option<BallData>,
    ) -> Result<State, Error> {
        self.ball.position = GameConfig::ball_start_pos();
        PhysicsHandler::update_pegs(&mut self.physics, &mut self.pegs)?;
        self.update_peg_generation()?;
        self.bucket
            .update::<{ GameConfig::WALL_LEFT }, { GameConfig::WALL_RIGHT }>();
        self.update_points_pres();

        PhysicsHandler::hide_non_collidable_pegs(&mut self.pegs);

        if bucketed_ball.is_some() {
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

    pub fn update_effects(&mut self) {
        self.text_box.update();
        self.screen_shake.update(&mut self.rng);
        self.white_flash.update();
    }

    pub fn update_points_pres(&mut self) {
        self.point_pres.retain_mut(|pp| pp.update());
    }

    pub fn transition_state(&mut self, new_state: State) {
        self.state_manager.transition_to(new_state);
    }

    pub fn add_bucketed_ball(&mut self, ball_data: BallData) {
        self.inventory.push(ball_data);
    }

    pub fn show(&mut self, frame: &mut agb::display::GraphicsFrame) {
        self.background.update(&self.screen_shake);

        if self.white_flash.is_active() {
            self.background.show_white(frame);
        } else {
            self.render_game_objects(frame);
        }
    }

    fn render_game_objects(&mut self, frame: &mut agb::display::GraphicsFrame) {
        self.pegs.show(frame);
        self.ball.show(frame);
        self.bucket.show(frame);
        self.background.show_game(frame);
        self.base_counter.show(frame);
        self.mult_counter.show(frame);
        self.coin_counter.show(frame);
        self.inventory_presenter.show(
            frame,
            &self.inventory,
            self.selected_inventory_index,
            matches!(self.state_manager.current(), State::InInventory),
        );
        self.text_box.show(frame);
        self.jauge.show(frame);

        for pp in self.point_pres.iter_mut() {
            pp.show(frame);
        }

        if matches!(self.state_manager.current(), State::Aiming) {
            self.launcher.show(frame);
            self.power_jauge.show(frame);
        }
    }

    fn process_peg_collisions(
        &mut self,
        touched: &[usize],
    ) -> Result<(), Error> {
        let mut touched_green_pegs: Vec<usize> = Vec::new();

        for &t in touched {
            let peg_kind = self.pegs.kind[t];
            self.pegs.collidable[t] = false;
            self.pegs.showable[t] = false;

            let score = self.score_manager.process_peg_hit(
                self.pegs.kind[t],
                &self.inventory,
                &self.current_ball_data,
                &mut self.mult_counter,
                &mut self.base_counter,
                &mut self.coin_counter,
            );

            if let Some(pp) =
                PointsPres::new(self.pegs.positions[t], score, peg_kind)
            {
                self.point_pres.push(pp);
            }

            if matches!(peg_kind, super::peg::Kind::Green) {
                touched_green_pegs.push(t);
            }
        }

        if !touched.is_empty() {
            self.screen_shake.start(
                GameConfig::SHAKE_DURATION,
                GameConfig::SHAKE_INTENSITY - 2,
            );
        }

        for t in touched_green_pegs {
            let peg_position = self.pegs.positions[t];
            self.peg_generators
                .push(PegGenerator::new(10, 10, peg_position));
        }

        Ok(())
    }

    fn update_peg_generation(&mut self) -> Result<(), Error> {
        for generator in &mut self.peg_generators {
            if generator.update() {
                PhysicsHandler::spawn_single_peg_from_green(
                    &mut self.pegs,
                    &mut self.physics,
                    generator.position(),
                    &mut self.rng,
                )?;
            }
        }

        self.peg_generators
            .retain(|generator| !generator.is_finished());
        Ok(())
    }

    fn start_end_effects(&mut self) {
        self.screen_shake
            .start(GameConfig::SHAKE_DURATION, GameConfig::SHAKE_INTENSITY);
        self.white_flash.start(GameConfig::FLASH_DURATION);
    }
}
