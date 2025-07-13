use super::effect::BallData;
use crate::error::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Aiming,
    Falling,
    InInventory,
    Counting { bucketed: Option<BallData> },
}

pub struct StateManager {
    current: State,
    previous: Option<State>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            current: State::Aiming,
            previous: None,
        }
    }

    pub fn current(&self) -> State {
        self.current
    }

    pub fn previous(&self) -> Option<State> {
        self.previous
    }

    pub fn transition_to(&mut self, new_state: State) {
        if new_state != self.current {
            self.previous = Some(self.current);
        }
        self.current = new_state;
    }

    pub fn return_to_previous(&self) -> Result<State, Error> {
        self.previous.ok_or(Error::NoLastState)
    }
}

