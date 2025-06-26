use crate::error::Error;

pub mod game;

enum Scene {
    Game,
}

pub fn main(mut gba: agb::Gba) -> Result<(), Error> {
    let mut state = Scene::Game;

    loop {
        state = match state {
            Scene::Game => game::main(&mut gba)?,
        };
    }
}
