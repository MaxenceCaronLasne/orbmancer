use super::physics::Velocity;
use super::render;
use super::render::Showable;
use crate::position::Position;
use crate::scenes::ecs::physics::Kinetic;
use agb::include_aseprite;
use bevy::prelude::*;

include_aseprite!(
    mod sprites,
    "assets/ball.aseprite"
);

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    label: Ball,
    kinematic: Kinetic,
    position: Position,
    velocity: Velocity,
    showable: Showable,
}

pub struct BallPlugin {
    pub initial_position: (i32, i32),
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        let e = app
            .world_mut()
            .spawn(BallBundle {
                label: Ball,
                kinematic: Kinetic,
                position: Position::new(
                    self.initial_position.0,
                    self.initial_position.1,
                ),
                velocity: Velocity::zero(),
                showable: Showable,
            })
            .id();

        render::add(app.world_mut(), e, sprites::BALL.sprite(0));
    }
}
