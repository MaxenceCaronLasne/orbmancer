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

    pub fn from_i32(value: i32) -> BallKind {
        match value {
            0 => BallKind::Identity,
            1 => BallKind::TheDoubler,
            2 => BallKind::SmallGrabber,
            _ => BallKind::Identity, // Default case
        }
    }

    pub fn rand(rng: &mut RandomNumberGenerator) -> BallKind {
        let v = rng.next_i32() as usize;
        BallKind::DROPPABLES[v % BallKind::NB_DROPPABLES]
    }
}

pub struct Save {
    inventory: Vec<BallKind, 16>,
    coins: i32,
}

impl Save {
    pub fn new() -> Self {
        let mut vec = Vec::new();
        let _ = vec.push(BallKind::TheDoubler);
        let _ = vec.push(BallKind::Identity);
        let _ = vec.push(BallKind::Identity);
        let _ = vec.push(BallKind::Identity);
        let _ = vec.push(BallKind::Identity);
        let _ = vec.push(BallKind::Identity);
        let _ = vec.push(BallKind::Identity);
        let _ = vec.push(BallKind::Identity);
        let _ = vec.push(BallKind::Identity);
        let _ = vec.push(BallKind::SmallGrabber);

        Self {
            inventory: vec,
            coins: 0,
        }
    }

    pub fn push_ball(&mut self, ball: BallKind) {
        self.inventory.push(ball).unwrap_or_else(|_| {
            println!("Full inventory");
        });
    }

    pub fn inventory(&self) -> &Vec<BallKind, 16> {
        &self.inventory
    }

    pub fn coins(&self) -> i32 {
        self.coins
    }
}
