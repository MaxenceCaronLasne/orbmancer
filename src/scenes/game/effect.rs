use crate::save::BallKind;
use crate::scenes::game::score::Score;
use alloc::vec::Vec;
use heapless::Vec as HeaplessVec;

use crate::peg::Kind;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BallData {
    kind: BallKind,
    active: ActiveEffect,
    passive: PassiveEffect,
}

impl BallData {
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            kind: BallKind::Identity,
            active: ActiveEffect::Identity,
            passive: PassiveEffect::Identity,
        }
    }

    pub fn from_kind(kind: BallKind) -> Self {
        match kind {
            BallKind::Identity => Self {
                kind,
                active: ActiveEffect::Identity,
                passive: PassiveEffect::Identity,
            },
            BallKind::TheDoubler => Self {
                kind,
                active: ActiveEffect::AddMult(1),
                passive: PassiveEffect::Identity,
            },
            BallKind::SmallGrabber => Self {
                kind,
                active: ActiveEffect::AddBase(10),
                passive: PassiveEffect::Identity,
            },
        }
    }

    pub fn active(&self) -> ActiveEffect {
        self.active
    }

    pub fn passive(&self) -> PassiveEffect {
        self.passive
    }

    pub fn kind(&self) -> BallKind {
        self.kind
    }
}

pub fn from_kinds(kinds: &HeaplessVec<BallKind, 10>) -> Vec<BallData> {
    kinds
        .iter()
        .map(|kind| BallData::from_kind(*kind))
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PassiveEffect {
    Identity,
    #[allow(dead_code)]
    AddMult(i32),
    #[allow(dead_code)]
    AddBase(i32),
}

impl PassiveEffect {
    pub fn apply(self, score: Score) -> Score {
        match self {
            PassiveEffect::Identity => score,
            PassiveEffect::AddMult(m) => score.add(0, m, 0),
            PassiveEffect::AddBase(b) => score.add(b, 0, 0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActiveEffect {
    Identity,
    AddMult(i32),
    #[allow(dead_code)]
    AddBase(i32),
}

impl ActiveEffect {
    pub fn apply(self, score: Score, kind: Kind) -> Score {
        match (self, kind) {
            (ActiveEffect::Identity, _) => score,
            (ActiveEffect::AddMult(m), Kind::Red) => score.add(0, m, 0),
            (ActiveEffect::AddBase(b), Kind::Blue) => score.add(b, 0, 0),
            (_, _) => score,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BucketEffect {
    Identity,
    #[allow(dead_code)]
    MultiplyMult(i32),
}

impl BucketEffect {
    pub fn apply(self, score: Score) -> Score {
        match self {
            BucketEffect::Identity => score,
            BucketEffect::MultiplyMult(m) => score.mult(1, m, 1),
        }
    }
}
