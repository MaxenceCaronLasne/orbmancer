use super::score::Score;

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
