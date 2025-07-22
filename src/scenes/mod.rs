use crate::error::Error;
use crate::level::Level;
use crate::save::Save;

mod drop;
mod game;
mod game_over;
mod title;

pub enum Scene {
    Title,
    Game,
    GameOver,
    Drop,
}

pub fn main(mut gba: agb::Gba, mut save: Save) -> Result<(), Error> {
    let mut state = Scene::Title;

    loop {
        state = match state {
            Scene::Title => title::main(&mut gba)?,
            Scene::Game => {
                game::main(&mut gba, &mut save, Level::new_test_level())?
            }
            Scene::GameOver => game_over::main(&mut gba, &mut save)?,
            Scene::Drop => drop::main(&mut gba, &mut save)?,
        };
    }
}
