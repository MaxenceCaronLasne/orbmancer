use agb::{println, rng::RandomNumberGenerator};
use heapless::Vec;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BallKind {
    Identity = 0,
    TheDoubler,
    SmallGrabber,
}

impl BallKind {
    pub const NB_DROPPABLES: usize = 2;
    const DROPPABLES: [BallKind; BallKind::NB_DROPPABLES] =
        [BallKind::TheDoubler, BallKind::SmallGrabber];

    pub fn sprite(&self) -> usize {
        match self {
            BallKind::Identity => 0,
            BallKind::TheDoubler => 0,
            BallKind::SmallGrabber => 0,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            BallKind::Identity => "just a ball...",
            BallKind::TheDoubler => "The Doubler",
            BallKind::SmallGrabber => "The Small Grabber",
        }
    }

    pub fn rand(rng: &mut RandomNumberGenerator) -> BallKind {
        let v = rng.next_i32() as usize;
        BallKind::DROPPABLES[v % BallKind::NB_DROPPABLES]
    }
}

pub struct Save {
    inventory: Vec<BallKind, 10>,
    coins: i32,
    level: i32,
}

impl Save {
    pub fn new() -> Self {
        let mut vec = Vec::new();
        let _ = vec.push(BallKind::Identity);
        let _ = vec.push(BallKind::Identity);
        let _ = vec.push(BallKind::Identity);
        let _ = vec.push(BallKind::Identity);
        let _ = vec.push(BallKind::Identity);

        Self {
            inventory: vec,
            coins: 0,
            level: 1,
        }
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn increase_level(&mut self) {
        self.level += 1;
    }

    pub fn push_ball(&mut self, ball: BallKind) {
        self.inventory.push(ball).unwrap_or_else(|_| {
            println!("Full inventory");
        });
    }

    pub fn inventory(&self) -> &Vec<BallKind, 10> {
        &self.inventory
    }

    pub fn coins(&self) -> i32 {
        self.coins
    }
}
