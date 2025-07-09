use crate::Fixed;
use crate::Coordinates;
use agb::display::GraphicsFrame;
use agb::display::font::{Layout, AlignmentKind, ObjectTextRenderer};
use agb::display::object::{Size, Object};
use agb::{
    fixnum::{num, vec2},
    include_aseprite,
};
use crate::scenes::game::effect::BallData;
use crate::text::{PALETTE, FONT};
use alloc::vec::Vec;

include_aseprite!(
    mod sprites,
    "assets/ball.aseprite"
);

pub struct InventoryPresenter {
    pub position: Coordinates,
    pub text_position: Coordinates,
    layout: Option<Layout>,
    letter_groups: Vec<Object>,
}

impl InventoryPresenter {
    pub fn new(position: Coordinates) -> Self {
        Self {
            position,
            text_position: vec2(num!(189), num!(2)),
            layout: None,
            letter_groups: Vec::new(),
        }
    }

    pub fn select(&mut self, ball_data: &BallData) {
        self.remove();
        let binding = ball_data.kind();
        let text = binding.description();
        let layout = Layout::new(text, &FONT, AlignmentKind::Left, 16, 46);
        self.layout = Some(layout);
    }

    pub fn update(&mut self) {
        let render = ObjectTextRenderer::new(PALETTE.into(), Size::S16x16);
        if let Some(layout) = &mut self.layout {
            if let Some(letters) = layout.next() {
                self.letter_groups.push(render.show(&letters, self.text_position.round()));
            }
        }
    }

    pub fn remove(&mut self) {
        self.layout = None;
        self.letter_groups.clear();
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame, inventory: &[BallData]) {
        for (i, bd) in inventory.iter().enumerate() {
            let spr_id = bd.kind().sprite();
            let mut sprite = agb::display::object::Object::new(sprites::BALL.sprite(spr_id));
            let position = self.position + vec2(num!(0), Fixed::new(10 * i as i32));
            sprite.set_pos(position.round()).show(frame);
        }

        for lg in &self.letter_groups {
            lg.show(frame);
        }

    }
}
