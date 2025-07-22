use agb::display::tiled::{
    RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER,
};
use agb::include_background_gfx;
use agb::input::{Button, ButtonController};

use crate::error::Error;
use crate::scenes::Scene;

include_background_gfx!(
    mod background,
    "000000",
    BACKGROUND => deduplicate "assets/title_screen.aseprite",
);

pub fn main(gba: &mut agb::Gba) -> Result<Scene, Error> {
    let mut gfx = gba.graphics.get();
    let mut input = ButtonController::new();

    crate::reset::reset_input(Button::A, &mut input, &mut gfx);

    VRAM_MANAGER.set_background_palettes(background::PALETTES);
    let mut bg = RegularBackground::new(
        agb::display::Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );
    bg.fill_with(&background::BACKGROUND);

    loop {
        input.update();

        if input.is_just_pressed(Button::A) {
            return Ok(Scene::Game);
        }

        let mut frame = gfx.frame();
        bg.show(&mut frame);
        frame.commit();
    }
}
