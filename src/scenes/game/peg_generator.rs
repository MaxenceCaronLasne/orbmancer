use crate::Coordinates;

pub struct PegGenerator {
    position: Coordinates,
    remaining_count: u8,
    generation_rate: u32,
    timer: u32,
}

impl PegGenerator {
    pub fn new(count: u8, generation_rate: u32, position: Coordinates) -> Self {
        Self {
            position,
            remaining_count: count,
            generation_rate,
            timer: 0,
        }
    }

    pub fn update(&mut self) -> bool {
        if self.remaining_count == 0 {
            return false;
        }

        self.timer += 1;

        if self.timer >= self.generation_rate {
            self.timer = 0;
            self.remaining_count -= 1;
            return true;
        }

        false
    }

    pub fn is_finished(&self) -> bool {
        self.remaining_count == 0
    }

    pub fn position(&self) -> Coordinates {
        self.position
    }
}