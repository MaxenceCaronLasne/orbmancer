use crate::error::Error;
use crate::save::Save;

pub mod game;

pub enum Scene {
    Game,
    GameOver,
    Win,
}

pub fn main(mut gba: agb::Gba, mut save: Save) -> Result<(), Error> {
    let mut state = Scene::Game;

    loop {
        state = match state {
            Scene::Game => game::main(&mut gba, &mut save)?,
            Scene::GameOver => {
                agb::println!("Game Over!");
                loop {
                    agb::halt();
                }
            }
            Scene::Win => {
                agb::println!("You Win!");
                loop {
                    agb::halt();
                }
            }
        };
    }
}
