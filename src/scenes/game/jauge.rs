use crate::{Coordinates, Fixed};
use agb::display::{AffineMatrix, GraphicsFrame};
use agb::{
    display::object::{AffineMatrixObject, AffineMode, Object, ObjectAffine},
    fixnum::{num, vec2},
    include_aseprite,
};

const PADDING: i32 = 3;

include_aseprite!(
    mod sprites,
    "assets/jauge.aseprite",
    "assets/jauge_foreground_left.aseprite",
    "assets/jauge_foreground_right.aseprite",
);

pub struct Jauge<const MIN: i32, const MAX: i32> {
    position: Coordinates,
    value: i32,
    foreground_sprite: (Object, Object),
    mask_sprite: (Object, Object),
}

impl<const MIN: i32, const MAX: i32> Jauge<MIN, MAX> {
    pub fn new(position: Coordinates) -> Self {
        Self {
            position,
            value: MAX,
            foreground_sprite: (
                Object::new(sprites::LEFT.sprite(0)),
                Object::new(sprites::RIGHT.sprite(0)),
            ),
            mask_sprite: (
                Object::new(sprites::BAR.sprite(0)),
                Object::new(sprites::BAR.sprite(0)),
            ),
        }
    }

    pub fn set(&mut self, value: i32, min: i32, max: i32) {
        let clamped = value.clamp(min, max);
        let normalized = Fixed::new(clamped - min) / Fixed::new(max - min);
        self.value = (normalized * num!(MAX - MIN)).floor() + MIN;
    }

    fn show_foreground(&mut self, frame: &mut GraphicsFrame) {
        const SPRL: i32 = 32;

        self.foreground_sprite
            .0
            .set_pos(self.position.round())
            .show(frame);

        self.foreground_sprite
            .1
            .set_pos(self.position.round() + vec2(SPRL, 0))
            .show(frame);
    }

    fn show_mask(&mut self, frame: &mut GraphicsFrame) {
        const SPRL: i32 = 32;
        let pos = self.position.round() + vec2(self.value + PADDING, 0);

        self.mask_sprite.0.set_pos(pos).show(frame);
        self.mask_sprite.1.set_pos(pos + vec2(SPRL, 0)).show(frame);
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame) {
        self.show_foreground(frame);
        self.show_mask(frame);
    }
}
