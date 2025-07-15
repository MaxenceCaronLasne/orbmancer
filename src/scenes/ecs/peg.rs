use super::physics::{Static, Velocity};
use super::render;
use super::render::Showable;
use crate::position::Position;
use crate::scenes::ecs::physics::Field;
use agb::include_aseprite;
use bevy::prelude::*;

include_aseprite!(
    mod sprites,
    "assets/peg.aseprite"
);

#[derive(Component)]
struct Peg;

#[derive(Bundle)]
struct PegBundle {
    label: Peg,
    physics: Static,
    field: Field,
    position: Position,
    velocity: Velocity,
    showable: Showable,
}

pub struct PegsPlugin {
    pub quantity: usize,
    pub grid_start_x: i32,
    pub grid_start_y: i32,
    pub grid_spacing_x: i32,
    pub grid_spacing_y: i32,
    pub grid_width: i32,
}

impl PegsPlugin {
    pub fn new_grid(
        quantity: usize,
        start_x: i32,
        start_y: i32,
        spacing_x: i32,
        spacing_y: i32,
        width: i32,
    ) -> Self {
        Self {
            quantity,
            grid_start_x: start_x,
            grid_start_y: start_y,
            grid_spacing_x: spacing_x,
            grid_spacing_y: spacing_y,
            grid_width: width,
        }
    }

    pub fn new_random(
        quantity: usize,
        min_x: i32,
        max_x: i32,
        min_y: i32,
        max_y: i32,
    ) -> Self {
        Self {
            quantity,
            grid_start_x: min_x,
            grid_start_y: min_y,
            grid_spacing_x: max_x - min_x,
            grid_spacing_y: max_y - min_y,
            grid_width: 0, // Special value to indicate random generation
        }
    }
}

impl Plugin for PegsPlugin {
    fn build(&self, app: &mut App) {
        if self.grid_width == 0 {
            // Random generation mode
            let mut positions = Vec::new();
            {
                let mut rng = app.world_mut().get_non_send_resource_mut::<agb::rng::RandomNumberGenerator>()
                    .expect("RandomNumberGenerator resource not found");

                for _ in 0..self.quantity {
                    let x = self.grid_start_x
                        + (rng.next_i32().abs() % self.grid_spacing_x);
                    let y = self.grid_start_y
                        + (rng.next_i32().abs() % self.grid_spacing_y);
                    positions.push((x, y));
                }
            }

            for (x, y) in positions {
                let e = app
                    .world_mut()
                    .spawn(PegBundle {
                        label: Peg,
                        physics: Static,
                        field: Field,
                        position: Position::new(x, y),
                        velocity: Velocity::zero(),
                        showable: Showable,
                    })
                    .id();

                render::add(app.world_mut(), e, sprites::RED.sprite(0));
            }
        } else {
            // Grid generation mode
            let mut pegs_created = 0;
            let mut row = 0;

            while pegs_created < self.quantity {
                let pegs_in_row =
                    (self.grid_width / self.grid_spacing_x).max(1);

                for col in 0..pegs_in_row {
                    if pegs_created >= self.quantity {
                        break;
                    }

                    let x = self.grid_start_x + col * self.grid_spacing_x;
                    let y = self.grid_start_y + row * self.grid_spacing_y;

                    let e = app
                        .world_mut()
                        .spawn(PegBundle {
                            label: Peg,
                            physics: Static,
                            field: Field,
                            position: Position::new(x, y),
                            velocity: Velocity::zero(),
                            showable: Showable,
                        })
                        .id();

                    render::add(app.world_mut(), e, sprites::RED.sprite(0));
                    pegs_created += 1;
                }

                row += 1;
            }
        }
    }
}
