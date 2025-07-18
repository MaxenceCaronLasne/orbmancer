use crate::Coordinates;
use crate::Fixed;
use crate::scenes::game::effect::BallData;
use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::{
    fixnum::{num, vec2},
    include_aseprite,
};

use super::config::GameConfig;

include_aseprite!(
    mod sprites,
    "assets/ball.aseprite",
    "assets/ball_cursor.aseprite",
);

pub struct InventoryPresenter {
    pub position: Coordinates,
}

impl InventoryPresenter {
    pub fn new(position: Coordinates) -> Self {
        Self { position }
    }

    pub fn update(&mut self) {}

    pub fn show(
        &mut self,
        frame: &mut GraphicsFrame,
        inventory: &[BallData],
        selected: usize,
        show_cursor: bool,
    ) {
        for (i, bd) in inventory.iter().enumerate() {
            let spr_id = bd.kind().sprite();
            let mut sprite =
                agb::display::object::Object::new(sprites::BALL.sprite(spr_id));
            let position = self.position
                + vec2(
                    num!(0),
                    Fixed::new(GameConfig::INVENTORY_ITEM_SPACING * i as i32),
                );
            sprite.set_pos(position.round()).show(frame);

            if show_cursor && i == selected {
                let mut cursor = Object::new(sprites::CURSOR.sprite(0));
                cursor.set_pos(position.round() + vec2(-4, -4)).show(frame);
            }
        }
    }
}
