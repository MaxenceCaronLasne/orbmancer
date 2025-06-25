use agb::display::GraphicsFrame;
use agb::fixnum::num;
use agb::{display::object::Object, fixnum::vec2, include_aseprite};

use crate::types::{Coordinate, Fixed, Force};

include_aseprite!(
    mod sprites,
    "assets/ball.aseprite"
);

pub fn radius() -> Fixed {
    num!(3)
}

pub struct Ball {
    pub position: Coordinate,
    pub velocity: Force,
    sprite: Object,
}

impl Ball {
    pub fn new(position: Coordinate) -> Self {
        Self {
            position,
            velocity: vec2(num!(0), num!(0)),
            sprite: Object::new(sprites::BALL.sprite(0)),
        }
    }

    pub fn show(self: &mut Ball, frame: &mut GraphicsFrame) {
        self.sprite.set_pos(self.position.round()).show(frame);
    }
}
