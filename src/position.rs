use crate::{Coordinates, Fixed};
use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Position(pub Coordinates);

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position(Coordinates::new(Fixed::new(x), Fixed::new(y)))
    }
}
