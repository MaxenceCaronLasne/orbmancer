use crate::{Coordinates, Fixed, Force};
use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::fixnum::{num, vec2};
use agb::include_aseprite;

pub const RADIUS: i32 = 3;
pub const FORCE_RADII: [f32; 4] = [5.0, 10.0, 15.0, 20.0];

include_aseprite!(
    mod sprites,
    "assets/peg.aseprite"
);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Blue,
    Red,
    Yellow,
}

pub struct Pegs<const N: usize> {
    pub positions: [Coordinates; N],
    pub velocities: [Force; N],
    pub force_radius_squared: [Fixed; N],
    pub sprite: [Object; N],
    pub showable: [bool; N],
    pub collidable: [bool; N],
    pub kind: [Kind; N],
}

impl<const N: usize> Pegs<N> {
    pub fn new(
        positions: [Coordinates; N],
        force_radius_squared: [Fixed; N],
        showable: [bool; N],
        collidable: [bool; N],
        kind: [Kind; N],
    ) -> Self {
        Self {
            positions,
            velocities: [vec2(num!(0.0), num!(0.0)); N],
            force_radius_squared,
            sprite: core::array::from_fn(|_| {
                Object::new(sprites::PEG.sprite(0))
            }),
            showable,
            collidable,
            kind,
        }
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame) {
        for i in 0..N {
            if self.showable[i] {
                self.sprite[i].set_pos(self.positions[i].round());
                self.sprite[i].show(frame);
            }
        }
    }
}
