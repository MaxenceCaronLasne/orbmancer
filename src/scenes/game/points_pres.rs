use super::peg::Kind;
use super::score::Score;
use agb::{
    display::{
        GraphicsFrame,
        object::{Object, Sprite},
    },
    fixnum::{Vector2D, num, vec2},
    include_aseprite,
};
use alloc::vec::Vec;

use crate::Coordinates;

include_aseprite!(
    mod sprites,
    "assets/points.aseprite",
    "assets/point_background.aseprite",
);

fn get_bg_sprite(kind: Kind) -> &'static Sprite {
    match kind {
        Kind::Blue => sprites::PTBG.sprite(0),
        Kind::Red => sprites::PTBG.sprite(1),
        Kind::Yellow => sprites::PTBG.sprite(2),
        Kind::Green => sprites::PTBG.sprite(2),
    }
}

fn get_value(score: Score, kind: Kind) -> Option<i32> {
    match kind {
        Kind::Blue => Some(score.base),
        Kind::Red => Some(score.mult),
        Kind::Yellow => Some(score.coins),
        Kind::Green => None,
    }
}

pub struct PointsPres {
    base_position: Coordinates,
    sprites: Vec<(Object, Object)>,
    frames: i32,
}

impl PointsPres {
    pub fn new(
        position: Coordinates,
        score: Score,
        kind: Kind,
    ) -> Option<Self> {
        let value = get_value(score, kind)?;
        let mut sprites = Vec::new();

        let mut div = 1;
        for _ in 0..(value / 10 + 1) {
            let spr_id = (value / div) % 10;
            sprites.push((
                Object::new(sprites::DIGITS.sprite(spr_id as usize)),
                Object::new(get_bg_sprite(kind)),
            ));
            div *= 10;
        }

        agb::println!("Score: {:?}", score);

        Some(Self {
            base_position: position,
            sprites,
            frames: 20,
        })
    }

    pub fn update(&mut self) -> bool {
        self.frames -= 1;
        self.base_position -= vec2(num!(0), num!(0.25));
        self.frames > 0
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame) {
        for (i, (nb, bg)) in self.sprites.iter_mut().rev().enumerate() {
            let padding = Vector2D::new(i as i32 * 4, 0);
            nb.set_pos(self.base_position.round() + padding);
            bg.set_pos(self.base_position.round() + padding);
            nb.show(frame);
            bg.show(frame);
        }
    }
}
