use agb::display::GraphicsFrame;
use agb::fixnum::num;
use agb::{display::object::Object, fixnum::vec2, include_aseprite};

use crate::{Coordinates, Force};

pub const RADIUS: i32 = 4;

include_aseprite!(
    mod sprites,
    "assets/ball.aseprite"
);

pub struct Ball {
    pub position: Coordinates,
    pub velocity: Force,
    sprite: Object,
}

impl Ball {
    pub fn new(position: Coordinates) -> Self {
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
