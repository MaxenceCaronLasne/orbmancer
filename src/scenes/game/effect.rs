use crate::save::BallKind;
use alloc::vec::Vec;
use heapless::Vec as HeaplessVec;

#[derive(Clone, Copy, Debug)]
pub struct BallData {
    active: ActiveEffect,
    passive: PassiveEffect,
}

impl BallData {
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            active: ActiveEffect::Identity,
            passive: PassiveEffect::Identity,
        }
    }

    #[allow(dead_code)]
    pub fn new(active: ActiveEffect, passive: PassiveEffect) -> Self {
        Self { active, passive }
    }

    pub fn from_kind(kind: BallKind) -> Self {
        match kind {
            BallKind::Identity => Self {
                active: ActiveEffect::Identity,
                passive: PassiveEffect::Identity,
            },
            BallKind::TheDoubler => Self {
                active: ActiveEffect::AddMult(1),
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
}

pub fn from_kinds(kinds: &HeaplessVec<BallKind, 16>) -> Vec<BallData> {
    kinds.iter().map(|kind| BallData::from_kind(*kind)).collect()
}

#[derive(Clone, Copy, Debug)]
pub enum PassiveEffect {
    Identity,
    #[allow(dead_code)]
    AddMult(i32),
    #[allow(dead_code)]
    AddBase(i32),
}

impl PassiveEffect {
    pub fn apply(self, base: i32, mult: i32) -> (i32, i32) {
        match self {
            PassiveEffect::Identity => (base, mult),
            PassiveEffect::AddMult(m) => (base, mult + m),
            PassiveEffect::AddBase(b) => (base + b, mult),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ActiveEffect {
    Identity,
    AddMult(i32),
    #[allow(dead_code)]
    AddBase(i32),
}

impl ActiveEffect {
    pub fn apply(self, base: i32, mult: i32) -> (i32, i32) {
        match self {
            ActiveEffect::Identity => (base, mult),
            ActiveEffect::AddMult(m) => (base, mult + m),
            ActiveEffect::AddBase(b) => (base + b, mult),
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
    pub fn apply(self, base: i32, mult: i32) -> (i32, i32) {
        match self {
            BucketEffect::Identity => (base, mult),
            BucketEffect::MultiplyMult(m) => (base, mult * m),
        }
    }
}
