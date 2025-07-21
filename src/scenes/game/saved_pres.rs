use agb::{
    display::{GraphicsFrame, object::Object},
    fixnum::{num, vec2},
    include_aseprite,
};

use crate::Coordinates;

include_aseprite!(
    mod sprites,
    "assets/saved.aseprite",
);

pub struct SavedPres {
    base_position: Coordinates,
    sprite: Object,
    frames: i32,
}

impl SavedPres {
    pub fn new(position: Coordinates) -> Self {
        Self {
            base_position: position,
            sprite: Object::new(sprites::SAVED.sprite(0)),
            frames: 20,
        }
    }

    pub fn update(&mut self) -> bool {
        self.frames -= 1;
        self.base_position -= vec2(num!(0), num!(0.25));
        self.frames > 0
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame) {
        self.sprite.set_pos(self.base_position.round());
        self.sprite.set_priority(agb::display::Priority::P0);
        self.sprite.show(frame);
    }
}
