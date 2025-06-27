use crate::types::{Coordinate, Fixed};
use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::fixnum::{Vector2D, num, vec2};
use agb::include_aseprite;

pub const RADIUS: f32 = 3.0;
pub const FORCE_RADII: [f32; 4] = [5.0, 10.0, 15.0, 20.0];
pub const MAX_PEGS: usize = 50;

include_aseprite!(
    mod sprites,
    "assets/peg.aseprite"
);

pub type PegIndex = u8;

pub struct Pegs {
    position: [Coordinate; MAX_PEGS as usize],
    force_radius_squared: [Fixed; MAX_PEGS as usize],
    sprite: [Object; MAX_PEGS as usize],
    touched: [bool; MAX_PEGS as usize],
    present: [bool; MAX_PEGS as usize],
    pub count: u8,
}

impl Pegs {
    pub fn new() -> Self {
        Self {
            position: [vec2(num!(0.0), num!(0.0)); MAX_PEGS as usize],
            force_radius_squared: [num!(0.0); MAX_PEGS as usize],
            sprite: core::array::from_fn(|_| {
                Object::new(sprites::PEG.sprite(0))
            }),
            touched: [false; MAX_PEGS as usize],
            present: [false; MAX_PEGS as usize],
            count: 0,
        }
    }

    pub fn add_peg(
        &mut self,
        position: Coordinate,
        force_radius: Fixed,
    ) -> Option<PegIndex> {
        if (self.count as usize) < MAX_PEGS {
            self.position[self.count as usize] = position;
            self.force_radius_squared[self.count as usize] =
                force_radius * force_radius;
            self.sprite[self.count as usize] =
                Object::new(sprites::PEG.sprite(0));
            self.touched[self.count as usize] = false;
            self.present[self.count as usize] = true;
            self.count += 1;
            Some(self.count - 1)
        } else {
            None
        }
    }

    pub fn position(&self, index: PegIndex) -> Coordinate {
        self.position[index as usize]
    }

    pub fn set_position(&mut self, index: PegIndex, new_value: Coordinate) {
        self.position[index as usize] = new_value;
    }

    pub fn force_radius_squared(&self, index: PegIndex) -> Fixed {
        self.force_radius_squared[index as usize]
    }

    pub fn touch(&mut self, index: PegIndex) {
        if index < self.count {
            self.touched[index as usize] = true;
        }
    }

    pub fn is_touched(&self, index: PegIndex) -> bool {
        index < self.count && self.touched[index as usize]
    }

    pub fn show(&mut self, index: PegIndex, frame: &mut GraphicsFrame) {
        if index < self.count {
            self.sprite[index as usize]
                .set_pos(self.position[index as usize].round());
            if !self.touched[index as usize] {
                self.sprite[index as usize].show(frame);
            }
        }
    }

    pub fn is_present(&self, index: PegIndex) -> bool {
        self.present[index as usize]
    }

    pub fn set_present(&mut self, index: PegIndex, is_set: bool) {
        self.present[index as usize] = is_set
    }
}
