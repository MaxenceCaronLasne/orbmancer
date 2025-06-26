use crate::types::{Coordinate, Fixed};
use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::include_aseprite;

// Peg constants
pub const RADIUS: f32 = 3.0;

// Standard force radii for peg interactions
pub const FORCE_RADII: [f32; 4] = [5.0, 10.0, 15.0, 20.0];

include_aseprite!(
    mod sprites,
    "assets/peg.aseprite"
);

pub struct Peg {
    pub position: Coordinate,
    pub force_radius: Fixed,
    sprite: Object,
    touched: bool,
}

impl Peg {
    pub fn new(position: Coordinate, force_radius: Fixed) -> Self {
        Self {
            position,
            force_radius,
            sprite: Object::new(sprites::PEG.sprite(0)),
            touched: false,
        }
    }

    pub fn touch(self: &mut Peg) {
        self.touched = true;
    }

    pub fn is_touched(self: &Peg) -> bool {
        self.touched
    }

    pub fn show(self: &mut Peg, frame: &mut GraphicsFrame) {
        self.sprite.set_pos(self.position.round());
        if !self.touched {
            self.sprite.show(frame);
        }
    }
}
