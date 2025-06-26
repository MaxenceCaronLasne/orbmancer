use crate::types::{Coordinate, Fixed};
use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::fixnum::num;
use agb::include_aseprite;

// Peg constants
pub const RADIUS: f32 = 3.0;

include_aseprite!(
    mod sprites,
    "assets/peg.aseprite"
);

pub struct Peg {
    pub position: Coordinate,
    sprite: Object,
    touched: bool,
}

impl Peg {
    pub fn new(position: Coordinate) -> Self {
        Self {
            position,
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
