use super::text_box::{self, TextBox};
use agb::display::object::Object;
use agb::display::tiled::{
    RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER,
};
use agb::fixnum::vec2;
use agb::input::{Button, ButtonController};
use agb::rng::RandomNumberGenerator;
use agb::{include_aseprite, include_background_gfx, println};
use const_random::const_random;

use crate::error::Error;
use crate::save::{BallKind, Save};
use crate::scenes::Scene;

include_background_gfx!(
    mod background,
    "000000",
    BACKGROUND => deduplicate "assets/drop_background.aseprite",
);

include_aseprite!(
    mod sprites,
    "assets/drop_ball_cursor.aseprite"
);

const BENCH_SIZE: usize = 2;

pub fn main(gba: &mut agb::Gba, save: &mut Save) -> Result<Scene, Error> {
    let mut gfx = gba.graphics.get();
    let mut input = ButtonController::new();

    save.increase_level();

    crate::reset::reset_input(Button::A, &mut input, &mut gfx);

    VRAM_MANAGER.set_background_palettes(background::PALETTES);
    let mut bg = RegularBackground::new(
        agb::display::Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );
    bg.fill_with(&background::BACKGROUND);

    let mut rng = RandomNumberGenerator::new_with_seed([
        const_random!(u32),
        const_random!(u32),
        const_random!(u32),
        const_random!(u32),
    ]);

    let bench: [BallKind; BENCH_SIZE] =
        [BallKind::rand(&mut rng), BallKind::rand(&mut rng)];
    let mut selector: usize = 0;

    let mut up_text_box = TextBox::new(vec2(66, 52), 138);
    up_text_box.set_text(bench[0].description());

    let mut down_text_box = TextBox::new(vec2(66, 100), 138);
    down_text_box.set_text(bench[1].description());

    let mut selector_sprite = Object::new(sprites::CURSOR.sprite(0));
    selector_sprite.set_pos(vec2(36, 59));

    loop {
        input.update();

        up_text_box.update();
        down_text_box.update();

        if input.is_just_pressed(Button::A) {
            save.push_ball(bench[selector]);
            println!("pushed {:?}", bench[selector]);
            return Ok(Scene::Game);
        }

        if input.is_just_pressed(Button::UP) && selector > 0 {
            selector -= 1;
            selector_sprite.set_pos(vec2(36, 59));
        }
        if input.is_just_pressed(Button::DOWN) && selector < BENCH_SIZE - 1 {
            selector += 1;
            selector_sprite.set_pos(vec2(36, 107));
        }

        let mut frame = gfx.frame();
        selector_sprite.show(&mut frame);
        bg.show(&mut frame);
        up_text_box.show(&mut frame);
        down_text_box.show(&mut frame);
        frame.commit();
    }
}
