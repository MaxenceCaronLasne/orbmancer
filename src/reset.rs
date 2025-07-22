use agb::{
    display::Graphics,
    input::{Button, ButtonController},
};

pub fn reset_input(
    button: Button,
    input: &mut ButtonController,
    gfx: &mut Graphics,
) {
    input.update();
    if input.is_pressed(button) {
        while !input.is_released(button) {
            input.update();
            let frame = gfx.frame();
            frame.commit();
        }
        let frame = gfx.frame();
        frame.commit();
    }
}
