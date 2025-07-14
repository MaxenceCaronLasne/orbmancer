use crate::{Coordinates, Fixed};
use agb::display::GraphicsFrame;
use agb::{
    display::object::Object,
    fixnum::{num, vec2},
    include_aseprite,
};

use super::config::GameConfig;

include_aseprite!(
    mod sprites,
    "assets/bucket.aseprite"
);

pub struct Bucket {
    pub position: Coordinates,
    sprite: Object,
    direction: Fixed,
    speed: Fixed,
    pub walls: [(Coordinates, Coordinates); 2],
}

impl Bucket {
    pub fn new(position: Coordinates) -> Self {
        let walls = Self::calculate_walls(position);
        Self {
            position,
            sprite: Object::new(sprites::BUCKET.sprite(0)),
            direction: num!(1.0),
            speed: num!(GameConfig::BUCKET_SPEED),
            walls,
        }
    }

    fn calculate_walls(
        position: Coordinates,
    ) -> [(Coordinates, Coordinates); 2] {
        [
            (
                vec2(position.x, position.y),
                vec2(position.x, position.y + num!(GameConfig::BUCKET_HEIGHT)),
            ),
            (
                vec2(position.x + num!(GameConfig::BUCKET_WIDTH), position.y),
                vec2(
                    position.x + num!(GameConfig::BUCKET_WIDTH),
                    position.y + num!(GameConfig::BUCKET_HEIGHT),
                ),
            ),
        ]
    }

    pub fn update<const LEFT_WALL: i32, const RIGHT_WALL: i32>(&mut self) {
        self.position.x += self.direction * self.speed;

        if self.position.x
            <= num!(LEFT_WALL + GameConfig::BUCKET_WALL_OFFSET_LEFT)
        {
            self.direction = num!(1.0);
        } else if self.position.x
            >= num!(RIGHT_WALL - GameConfig::BUCKET_WALL_OFFSET_RIGHT)
        {
            self.direction = num!(-1.0);
        }

        self.walls = Self::calculate_walls(self.position);
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame) {
        self.sprite.set_pos(self.position.round()).show(frame);
    }

    pub fn is_in_bucket(&self, position: Coordinates) -> bool {
        self.position.x < position.x
            && position.x < self.position.x + GameConfig::BUCKET_WIDTH
            && self.position.y < position.y
            && position.y < self.position.y + GameConfig::BUCKET_HEIGHT
    }
}
