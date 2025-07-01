use crate::{Coordinates, Fixed, Force};
use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::fixnum::{num, vec2};
use agb::include_aseprite;

pub const RADIUS: i32 = 4;
pub const FORCE_RADII: [f32; 4] = [5.0, 10.0, 15.0, 20.0];
pub const MAX_PEGS: usize = 100;

include_aseprite!(
    mod sprites,
    "assets/peg.aseprite"
);

pub struct Pegs {
    pub positions: [Coordinates; MAX_PEGS],
    pub velocities: [Force; MAX_PEGS],
    pub force_radius_squared: [Fixed; MAX_PEGS],
    pub sprite: [Object; MAX_PEGS],
    pub showable: [bool; MAX_PEGS],
    pub collidable: [bool; MAX_PEGS],
}

impl Pegs {
    pub fn new(
        positions: [Coordinates; MAX_PEGS],
        force_radius_squared: [Fixed; MAX_PEGS],
        showable: [bool; MAX_PEGS],
        collidable: [bool; MAX_PEGS],
    ) -> Self {
        Self {
            positions,
            velocities: [vec2(num!(0.0), num!(0.0)); MAX_PEGS],
            force_radius_squared,
            sprite: core::array::from_fn(|_| {
                Object::new(sprites::PEG.sprite(0))
            }),
            showable,
            collidable,
        }
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame) {
        for i in 0..MAX_PEGS {
            if self.showable[i] {
                self.sprite[i].set_pos(self.positions[i].round());
                self.sprite[i].show(frame);
            }
        }
    }
}
