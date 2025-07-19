use crate::error::Error;
use heapless::Vec;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BallKind {
    Identity,
    TheDoubler,
    SmallGrabber,
}

impl BallKind {
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

    pub fn push_ball(&mut self, kind: BallKind) -> Result<(), Error> {
        self.inventory.push(kind).map_err(|_| Error::InventoryFull)
    }

    pub fn inventory(&self) -> &Vec<BallKind, 16> {
        &self.inventory
    }

    pub fn coins(&self) -> i32 {
        self.coins
    }
}
