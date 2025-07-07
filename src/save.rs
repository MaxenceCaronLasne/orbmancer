use crate::error::Error;
use heapless::Vec;

#[derive(Clone, Copy, Debug)]
pub enum BallKind {
    Identity,
    TheDoubler,
}

pub struct Save {
    inventory: Vec<BallKind, 16>, // Max 16 balls in inventory
}

impl Save {
    pub fn new() -> Self {
        Self {
            inventory: Vec::new(),
        }
    }

    pub fn push_ball(&mut self, kind: BallKind) -> Result<(), Error> {
        self.inventory.push(kind).map_err(|_| Error::InventoryFull)
    }

    pub fn inventory(&self) -> &Vec<BallKind, 16> {
        &self.inventory
    }
}

