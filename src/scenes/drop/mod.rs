use agb::display::tiled::{
    RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER,
};
use agb::input::{Button, ButtonController};
use agb::rng::RandomNumberGenerator;
use agb::{include_background_gfx, println};
use const_random::const_random;

use crate::error::Error;
use crate::save::{BallKind, Save};
use crate::scenes::Scene;

include_background_gfx!(
    mod background,
    "000000",
    BACKGROUND => deduplicate "assets/drop_screen.aseprite",
);

const BENCH_SIZE: usize = 2;

pub fn main(gba: &mut agb::Gba, save: &mut Save) -> Result<Scene, Error> {
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

    let mut rng = RandomNumberGenerator::new_with_seed([
        const_random!(u32),
        const_random!(u32),
        const_random!(u32),
        const_random!(u32),
    ]);

    let bench: [BallKind; BENCH_SIZE] =
        [BallKind::rand(&mut rng), BallKind::rand(&mut rng)];
    let mut selector: usize = 0;

    loop {
        input.update();

        if input.is_just_pressed(Button::A) {
            save.push_ball(bench[selector]);
            println!("pushed {:?}", bench[selector]);
            return Ok(Scene::Game);
        }

        if input.is_just_pressed(Button::LEFT) && selector > 0 {
            selector -= 1;
        }
        if input.is_just_pressed(Button::RIGHT) && selector < BENCH_SIZE - 1 {
            selector += 1;
        }

        let mut frame = gfx.frame();
        bg.show(&mut frame);
        frame.commit();
    }
}
