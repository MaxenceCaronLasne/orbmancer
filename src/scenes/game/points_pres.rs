use super::score::Score;
use agb::{
    display::{GraphicsFrame, object::Object},
    fixnum::{Vector2D, num, vec2},
    include_aseprite,
};
use alloc::vec::Vec;

use crate::Coordinates;

include_aseprite!(
    mod sprites,
    "assets/points.aseprite"
);

pub struct PointsPres {
    base_position: Coordinates,
    sprites: Vec<Object>,
    frames: i32,
}

impl PointsPres {
    pub fn new(position: Coordinates, score: Score) -> Self {
        let value = score.base;
        let mut sprites = Vec::new();

        let mut div = 1;
        for _ in 0..(value / 10 + 1) {
            let spr_id = (value / div) % 10;
            sprites.push(Object::new(sprites::DIGITS.sprite(spr_id as usize)));
            div *= 10;
        }

        agb::println!("Score: {:?}", score);

        Self {
            base_position: position,
            sprites,
            frames: 20,
        }
    }

    pub fn update(&mut self) -> bool {
        self.frames -= 1;
        self.base_position -= vec2(num!(0), num!(0.25));
        self.frames > 0
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame) {
        for (i, s) in self.sprites.iter_mut().rev().enumerate() {
            let padding = Vector2D::new(i as i32 * 4, 0);
            s.set_pos(self.base_position.round() + padding);
            s.show(frame);
        }
    }
}
