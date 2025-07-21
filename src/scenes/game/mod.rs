use crate::level::Level;
use crate::{error::Error, save::Save, scenes::Scene};
use agb::input::ButtonController;

mod background;
mod ball;
mod bucket;
mod config;
mod counter;
mod effect;
mod game_state;
mod input;
mod inventory;
mod jauge;
mod launcher;
mod peg;
mod peg_generator;
mod physics_handler;
mod points_pres;
mod saved_pres;
mod score;
mod shake;
mod state;
mod text_box;

#[cfg(test)]
mod test;

use game_state::GameState;
use state::State;

fn update<const MAX_PEGS: usize>(
    game_state: &mut GameState<MAX_PEGS>,
    input: &mut ButtonController,
) -> Result<Scene, Error> {
    input.update();
    game_state.update_effects();

    let new_state = match game_state.state() {
        State::Aiming => game_state.update_aiming(input)?,
        State::Falling => game_state.update_falling(input)?,
        State::InInventory => game_state.update_inventory(input)?,
        State::Counting { bucketed } => {
            crate::bench::log();
            let res = game_state.update_counting(bucketed)?;

            if let Some(ball_data) = bucketed {
                game_state.add_bucketed_ball(ball_data);
            }

            if game_state.is_winning() {
                return Ok(Scene::Win);
            }

            match game_state.pop_ball() {
                Ok(_) => {}
                Err(Error::NoBalls) => {
                    return Ok(Scene::GameOver);
                }
                Err(e) => {
                    return Err(e);
                }
            }

            game_state.set_text_to_current_ball();
            res
        }
    };

    game_state.transition_state(new_state);
    Ok(Scene::Game)
}

pub fn main(
    gba: &mut agb::Gba,
    save: &mut Save,
    level: Level,
) -> Result<Scene, Error> {
    let mut gfx = gba.graphics.get();
    let mut input = ButtonController::new();
    let mut timers = gba.timers.timers();

    let mut game_state = GameState::<50>::new(save, level)?;
    game_state.pop_ball()?;

    crate::bench::init(&mut timers);

    game_state.set_text_to_current_ball();

    loop {
        match update(&mut game_state, &mut input) {
            Ok(Scene::Game) => {}
            res => return res,
        }

        let mut frame = gfx.frame();
        game_state.show(&mut frame);
        frame.commit();
    }
}
