use crate::error::Error;

mod game;

enum State {
    Game,
}

pub fn main(mut gba: agb::Gba) -> Result<(), Error> {
    let mut state = State::Game;

    loop {
        state = match state {
            State::Game => game::main(&mut gba)?,
        };
    }
}
