use agb::display::Priority;
use agb::display::tiled::{
    RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER,
};
use agb::include_background_gfx;

include_background_gfx!(
    mod background,
    GAME => deduplicate "assets/background.aseprite"
);

pub fn new() -> RegularBackground {
    VRAM_MANAGER.set_background_palettes(background::PALETTES);

    let mut res = RegularBackground::new(
        Priority::P3,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    res.fill_with(&background::GAME);

    res
}
