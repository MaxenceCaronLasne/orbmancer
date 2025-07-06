use heapless::Vec;

#[derive(Clone, Copy, Debug)]
pub enum BallKind {
    Identity,
    TheDoubler,
}

pub struct Save {
    pub inventory: Vec<BallKind, 16>, // Max 16 balls in inventory
}

impl Save {
    pub fn new() -> Self {
        Self {
            inventory: Vec::new(),
        }
    }
    
    pub fn push_ball(&mut self, kind: BallKind) -> Result<(), ()> {
        self.inventory.push(kind).map_err(|_| ())
    }
    
    pub fn len(&self) -> usize {
        self.inventory.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.inventory.is_empty()
    }
}