use super::shake::ScreenShake;
use agb::display::tiled::{
    RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER,
};
use agb::display::{GraphicsFrame, Priority};
use agb::include_background_gfx;

include_background_gfx!(
    mod background,
    "000001",
    GAME => deduplicate "assets/background.aseprite",
    WHITE => deduplicate "assets/white_background.aseprite"
);

pub struct Background {
    game_background: RegularBackground,
    white_background: RegularBackground,
}

impl Background {
    pub fn new() -> Self {
        VRAM_MANAGER.set_background_palettes(background::PALETTES);

        let mut game_background = RegularBackground::new(
            Priority::P0,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        game_background.fill_with(&background::GAME);

        let mut white_background = RegularBackground::new(
            Priority::P0,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        white_background.fill_with(&background::WHITE);

        Self {
            game_background,
            white_background,
        }
    }

    pub fn update(&mut self, screen_shake: &ScreenShake) {
        if screen_shake.is_active() {
            self.game_background
                .set_scroll_pos(screen_shake.offset().round());
        } else {
            self.game_background.set_scroll_pos((0, 0));
        }
    }

    pub fn show_game(&mut self, frame: &mut GraphicsFrame) {
        self.game_background.show(frame);
    }

    pub fn show_white(&mut self, frame: &mut GraphicsFrame) {
        self.white_background.show(frame);
    }
}
