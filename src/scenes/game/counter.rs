use crate::Coordinates;
use agb::display::GraphicsFrame;
use agb::{
    fixnum::{num, vec2},
    include_aseprite,
};
use alloc::vec::Vec;

include_aseprite!(
    mod sprites,
    "assets/numbers.aseprite"
);

pub enum Alignment {
    LeftToRight,
    RightToLeft,
}

pub struct Counter {
    counter: i32,
    position: Coordinates,
    alignment: Alignment,
}

impl Counter {
    pub fn new(position: Coordinates, alignment: Alignment) -> Self {
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
            let mut sprite = agb::display::object::Object::new(sprites::NUMBERS.sprite(0));
            match self.alignment {
                Alignment::LeftToRight => sprite.set_pos(self.position.round()).show(frame),
                Alignment::RightToLeft => sprite.set_pos(self.position.round() - vec2(2, 0)).show(frame),
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
        
        let start_x = match self.alignment {
            Alignment::LeftToRight => self.position.x,
            Alignment::RightToLeft => self.position.x - agb::fixnum::num!(2.0),
        };
        
        for (i, &digit) in digits.iter().enumerate() {
            let mut sprite = agb::display::object::Object::new(sprites::NUMBERS.sprite(digit));
            let digit_pos = agb::fixnum::vec2(
                start_x + agb::fixnum::num!(4.0) * i as i32,
                self.position.y
            );
            sprite.set_pos(digit_pos.round()).show(frame);
        }
    }
}
