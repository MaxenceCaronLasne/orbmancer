use crate::Fixed;
use crate::{Coordinates};
use agb::display::{GraphicsFrame};
use agb::{
    fixnum::{num, vec2},
    include_aseprite,
};
use crate::scenes::game::effect::BallData;

include_aseprite!(
    mod sprites,
    "assets/ball.aseprite"
);

pub struct InventoryPresenter {
    pub position: Coordinates,
}

impl InventoryPresenter {
    pub fn new(position: Coordinates) -> Self {
        Self {
            position,
        }
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame, inventory: &[BallData]) {
        for (i, bd) in inventory.iter().enumerate() {
            let spr_id = bd.kind().sprite();
            let mut sprite = agb::display::object::Object::new(sprites::BALL.sprite(spr_id));
            let position = self.position + vec2(num!(0), Fixed::new(10 * i as i32));
            sprite.set_pos(position.round()).show(frame);
        }
    }
}
