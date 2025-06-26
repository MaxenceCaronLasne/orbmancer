use agb::display::GraphicsFrame;
use agb::fixnum::num;
use agb::{display::object::Object, fixnum::vec2, include_aseprite};

use crate::types::{Coordinate, Force};

// Ball constants
pub const RADIUS: f32 = 3.0;
const ZERO: f32 = 0.0;

include_aseprite!(
    mod sprites,
    "assets/ball.aseprite"
);

pub struct Ball {
    pub position: Coordinate,
    pub velocity: Force,
    sprite: Object,
}

impl Ball {
    pub fn new(position: Coordinate) -> Self {
        Self {
            position,
            velocity: vec2(num!(ZERO), num!(ZERO)),
            sprite: Object::new(sprites::BALL.sprite(0)),
        }
    }

    pub fn show(self: &mut Ball, frame: &mut GraphicsFrame) {
        self.sprite.set_pos(self.position.round()).show(frame);
    }
}
