use crate::types::Coordinate;
use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::include_aseprite;

include_aseprite!(
    mod sprites,
    "assets/peg.aseprite"
);

pub struct Peg {
    position: Coordinate,
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

    pub fn show(self: &mut Self, frame: &mut GraphicsFrame) {
        self.sprite.set_pos(self.position.round()).show(frame);
    }
}
