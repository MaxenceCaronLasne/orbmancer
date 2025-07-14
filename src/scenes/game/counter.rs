use crate::Coordinates;
use agb::display::GraphicsFrame;
use agb::display::font::AlignmentKind;
use agb::{fixnum::vec2, include_aseprite};
use alloc::vec::Vec;

use super::config::GameConfig;

include_aseprite!(
    mod sprites,
    "assets/numbers.aseprite"
);

pub struct Counter {
    counter: i32,
    position: Coordinates,
    alignment: AlignmentKind,
}

impl Counter {
    pub fn new(position: Coordinates, alignment: AlignmentKind) -> Self {
        Self {
            counter: 0,
            position,
            alignment,
        }
    }

    pub fn set(&mut self, value: i32) {
        self.counter = value;
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame) {
        let abs_counter = self.counter.abs();

        if abs_counter == 0 {
            let mut sprite =
                agb::display::object::Object::new(sprites::NUMBERS.sprite(0));
            match self.alignment {
                AlignmentKind::Left => {
                    sprite.set_pos(self.position.round()).show(frame)
                }
                AlignmentKind::Right => sprite
                    .set_pos(
                        self.position.round()
                            - vec2(
                                GameConfig::COUNTER_POSITION_OFFSET as i32,
                                0,
                            ),
                    )
                    .show(frame),
                _ => {}
            }
            return;
        }

        let mut digits = Vec::new();
        let mut num = abs_counter;
        while num > 0 {
            digits.push((num % 10) as usize);
            num /= 10;
        }
        digits.reverse();

        for (i, &digit) in digits.iter().enumerate() {
            let mut sprite = agb::display::object::Object::new(
                sprites::NUMBERS.sprite(digit),
            );
            let digit_pos = match self.alignment {
                AlignmentKind::Left => agb::fixnum::vec2(
                    self.position.x
                        + agb::fixnum::num!(GameConfig::COUNTER_DIGIT_SPACING)
                            * i as i32,
                    self.position.y,
                ),
                AlignmentKind::Right => agb::fixnum::vec2(
                    self.position.x
                        - agb::fixnum::num!(
                            GameConfig::COUNTER_POSITION_OFFSET
                        )
                        - agb::fixnum::num!(GameConfig::COUNTER_DIGIT_SPACING)
                            * (digits.len() - 1 - i) as i32,
                    self.position.y,
                ),
                _ => agb::fixnum::vec2(
                    self.position.x
                        + agb::fixnum::num!(GameConfig::COUNTER_DIGIT_SPACING)
                            * i as i32,
                    self.position.y,
                ),
            };
            sprite.set_pos(digit_pos.round()).show(frame);
        }
    }
}
