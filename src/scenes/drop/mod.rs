use agb::input::{Button, ButtonController};

use crate::error::Error;
use crate::save::Save;
use crate::scenes::Scene;

pub fn main(gba: &mut agb::Gba, save: &mut Save) -> Result<Scene, Error> {
    let mut gfx = gba.graphics.get();
    let mut input = ButtonController::new();
    let mut timers = gba.timers.timers();

    crate::bench::init(&mut timers);
    crate::reset::reset_input(Button::A, &mut input, &mut gfx);

    loop {
        input.update();

        if input.is_just_pressed(Button::A)
            || input.is_just_pressed(Button::START)
        {
            return Ok(Scene::Game);
        }

        let mut frame = gfx.frame();
        frame.commit();
    }
}
