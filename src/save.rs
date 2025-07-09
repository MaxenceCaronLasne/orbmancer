use crate::error::Error;
use heapless::Vec;

#[derive(Clone, Copy, Debug)]
pub enum BallKind {
    Identity,
    TheDoubler,
}

pub struct Save {
    inventory: Vec<BallKind, 16>,
    coins: i32,
}

impl Save {
    pub fn new() -> Self {
        Self {
            inventory: Vec::new(),
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

