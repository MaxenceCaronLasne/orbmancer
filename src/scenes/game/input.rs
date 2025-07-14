use agb::input::{Button, ButtonController};

pub struct InputHandler;

impl InputHandler {
    pub fn get_movement_direction(
        input: &ButtonController,
    ) -> MovementDirection {
        let left = input.is_pressed(Button::LEFT);
        let right = input.is_pressed(Button::RIGHT);

        match (left, right) {
            (true, false) => MovementDirection::Left,
            (false, true) => MovementDirection::Right,
            _ => MovementDirection::None,
        }
    }

    pub fn is_fire_pressed(input: &ButtonController) -> bool {
        input.is_just_pressed(Button::A)
    }

    pub fn is_fire_held(input: &ButtonController) -> bool {
        input.is_pressed(Button::A)
    }

    pub fn is_fire_released(input: &ButtonController) -> bool {
        input.is_just_released(Button::A)
    }

    pub fn is_inventory_pressed(input: &ButtonController) -> bool {
        input.is_just_pressed(Button::SELECT)
    }

    pub fn get_inventory_navigation(
        input: &ButtonController,
    ) -> InventoryNavigation {
        if input.is_just_pressed(Button::UP) {
            InventoryNavigation::Up
        } else if input.is_just_pressed(Button::DOWN) {
            InventoryNavigation::Down
        } else {
            InventoryNavigation::None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MovementDirection {
    Left,
    Right,
    None,
}

#[derive(Debug, PartialEq)]
pub enum InventoryNavigation {
    Up,
    Down,
    None,
}
