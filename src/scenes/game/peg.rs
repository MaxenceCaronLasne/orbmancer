use crate::types::{Coordinate, Fixed};
use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::fixnum::{num, vec2};
use agb::include_aseprite;

pub const RADIUS: f32 = 3.0;
pub const FORCE_RADII: [f32; 4] = [5.0, 10.0, 15.0, 20.0];
pub const MAX_PEGS: usize = 50;

include_aseprite!(
    mod sprites,
    "assets/peg.aseprite"
);

pub struct Pegs {
    pub positions: [Coordinate; MAX_PEGS],
    pub force_radii: [Fixed; MAX_PEGS],
    pub sprites: [Object; MAX_PEGS],
    pub touched: [bool; MAX_PEGS],
    pub present: [bool; MAX_PEGS],
    pub count: usize,
}

impl Pegs {
    pub fn new() -> Self {
        Self {
            positions: [vec2(num!(0.0), num!(0.0)); MAX_PEGS],
            force_radii: [num!(0.0); MAX_PEGS],
            sprites: core::array::from_fn(|_| {
                Object::new(sprites::PEG.sprite(0))
            }),
            touched: [false; MAX_PEGS],
            present: [false; MAX_PEGS],
            count: 0,
        }
    }

    pub fn add_peg(
        &mut self,
        position: Coordinate,
        force_radius: Fixed,
    ) -> Option<usize> {
        if self.count < MAX_PEGS {
            self.positions[self.count] = position;
            self.force_radii[self.count] = force_radius;
            self.sprites[self.count] = Object::new(sprites::PEG.sprite(0));
            self.touched[self.count] = false;
            self.present[self.count] = true;
            self.count += 1;
            Some(self.count - 1)
        } else {
            None
        }
    }

    pub fn touch(&mut self, index: usize) {
        if index < self.count {
            self.touched[index] = true;
        }
    }

    pub fn is_touched(&self, index: usize) -> bool {
        index < self.count && self.touched[index]
    }

    pub fn show(&mut self, index: usize, frame: &mut GraphicsFrame) {
        if index < self.count {
            self.sprites[index].set_pos(self.positions[index].round());
            if !self.touched[index] {
                self.sprites[index].show(frame);
            }
        }
    }
}
