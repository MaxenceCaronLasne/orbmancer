use crate::scenes::game::config::GameConfig;
use crate::scenes::game::counter::Counter;
use crate::scenes::game::effect::{BallData, BucketEffect};
use crate::scenes::game::peg::Kind;

pub type Component = i32;
pub type Damage = i32;
pub type Coins = i32;

#[derive(Clone, Copy, Debug)]
pub struct Score {
    pub base: Component,
    pub mult: Component,
    pub coins: Coins,
}

impl Score {
    pub fn new(base: Component, mult: Component, coins: Coins) -> Self {
        Self { base, mult, coins }
    }

    pub fn add(self, base: Component, mult: Component, coins: Coins) -> Self {
        Self {
            base: self.base + base,
            mult: self.mult + mult,
            coins: self.coins + coins,
        }
    }

    pub fn mult(self, base: Component, mult: Component, coins: Coins) -> Self {
        Self {
            base: self.base * base,
            mult: self.mult * mult,
            coins: self.coins * coins,
        }
    }

    pub fn apply(self, score: Score) -> Self {
        Self {
            base: self.base + score.base,
            mult: self.mult + score.mult,
            coins: self.coins + score.coins,
        }
    }

    pub fn extract(self) -> (Damage, Coins) {
        (self.base * self.mult, self.coins)
    }
}

pub struct ScoreManager {
    current_score: Option<Score>,
    damages: Damage,
    coins: Coins,
}

impl ScoreManager {
    pub fn new(coins: Coins) -> Self {
        Self {
            current_score: None,
            damages: 0,
            coins,
        }
    }

    pub fn process_peg_hit(
        &mut self,
        peg_kind: Kind,
        inventory: &[BallData],
        current_ball_data: &Option<BallData>,
        mult_counter: &mut Counter,
        base_counter: &mut Counter,
        coin_counter: &mut Counter,
    ) {
        let mut score = self.current_score.unwrap_or(Score::new(0, 1, 0));

        score = score.apply(match peg_kind {
            Kind::Blue => Score::new(1, 0, 0),
            Kind::Red => Score::new(0, 1, 0),
            Kind::Yellow => Score::new(0, 0, 1),
            Kind::Green => Score::new(0, 0, 0),
        });

        for pe in inventory {
            score = pe.passive().apply(score);
        }

        if let Some(ball_data) = current_ball_data {
            score = ball_data.active().apply(score);
        }

        mult_counter.set(score.mult);
        base_counter.set(score.base);
        coin_counter.set(self.coins + score.coins);
        self.current_score = Some(score);
    }

    pub fn process_bucket_bonus(
        &mut self,
        bucket_effects: &[BucketEffect],
        mult_counter: &mut Counter,
        base_counter: &mut Counter,
        coin_counter: &mut Counter,
    ) {
        let mut score = self.current_score.unwrap_or(Score::new(0, 1, 0));

        for e in bucket_effects {
            score = e.apply(score);
        }

        mult_counter.set(score.mult);
        base_counter.set(score.base);
        coin_counter.set(self.coins + score.coins);
        self.current_score = Some(score);
    }

    pub fn extract_final_score(&mut self) -> (Damage, Coins) {
        if let Some(score) = self.current_score {
            let (damages, coins) = score.extract();
            self.damages += damages;
            self.coins += coins;
            self.current_score = None;
            (self.damages, self.coins)
        } else {
            (self.damages, self.coins)
        }
    }

    pub fn is_winning(&self) -> bool {
        self.damages > GameConfig::TARGET_SCORE
    }

    pub fn reset_counters(
        &mut self,
        mult_counter: &mut Counter,
        base_counter: &mut Counter,
    ) {
        mult_counter.reset();
        base_counter.reset();
    }
}
