use super::score::Score;

#[derive(Clone, Copy, Debug)]
pub struct BallData {
    active: BallEffect,
    passive: BallEffect,
}

impl BallData {
    pub fn empty() -> Self {
        Self {
            active: BallEffect::Identity,
            passive: BallEffect::Identity,
        }
    }

    pub fn new(active: BallEffect, passive: BallEffect) -> Self {
        Self { active, passive }
    }

    pub fn active(&self) -> BallEffect {
        self.active
    }

    pub fn passive(&self) -> BallEffect {
        self.passive
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BallEffect {
    Identity,
    AddMult(i32),
    AddBase(i32),
}

impl BallEffect {
    pub fn apply(self, base: i32, mult: i32) -> (i32, i32) {
        match self {
            BallEffect::Identity => (base, mult),
            BallEffect::AddMult(m) => (base, mult + m),
            BallEffect::AddBase(b) => (base + b, mult),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BucketEffect {
    Identity,
    MultiplyMult(i32),
}

impl BucketEffect {
    pub fn apply(self, score: &mut Score) {
        match self {
            BucketEffect::Identity => (),
            BucketEffect::MultiplyMult(m) => score.mult *= m,
        }
    }
}
