use agb::display::font::Font;
use agb::{
    include_font,
};
use agb::display::{
    Palette16, Rgb15,
};

pub static PALETTE: &Palette16 = {
    let mut palette = [Rgb15::BLACK; 16];
    palette[1] = Rgb15::WHITE;
    &Palette16::new(palette)
};

pub static FONT: Font = include_font!("assets/m3x6.ttf", 16);
