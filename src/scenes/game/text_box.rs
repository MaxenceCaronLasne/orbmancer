use crate::text::{FONT, PALETTE};
use agb::display::GraphicsFrame;
use agb::display::font::{AlignmentKind, Layout, ObjectTextRenderer};
use agb::display::object::{Object, Size};
use agb::fixnum::Vector2D;
use alloc::vec::Vec;

const LETTER_GROUP_SIZE: i32 = 16;

pub struct TextBox {
    position: Vector2D<i32>,
    width: i32,
    layout: Option<Layout>,
    letters: Vec<Object>,
}

impl TextBox {
    pub fn new(position: Vector2D<i32>, width: i32) -> Self {
        Self {
            position,
            width,
            layout: None,
            letters: Vec::new(),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.remove();
        self.layout = Some(Layout::new(
            text,
            &FONT,
            AlignmentKind::Left,
            LETTER_GROUP_SIZE,
            self.width,
        ));
    }

    pub fn update(&mut self) {
        let render = ObjectTextRenderer::new(PALETTE.into(), Size::S16x16);
        if let Some(layout) = &mut self.layout
            && let Some(letters) = layout.next()
        {
            self.letters.push(render.show(&letters, self.position));
        }
    }

    pub fn remove(&mut self) {
        self.layout = None;
        self.letters.clear();
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame) {
        for lg in &self.letters {
            lg.show(frame);
        }
    }
}
