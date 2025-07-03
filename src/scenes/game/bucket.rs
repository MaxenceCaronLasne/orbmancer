use crate::{Coordinates, Fixed};
use agb::display::GraphicsFrame;
use agb::{
    display::object::Object,
    fixnum::{num, vec2},
    include_aseprite,
};

include_aseprite!(
    mod sprites,
    "assets/bucket.aseprite"
);

const MIN_X: f32 = 0.0;
const MAX_X: f32 = 140.0;
const SPEED: f32 = 1.0;
const BUCKET_HALF_WIDTH: f32 = 16.0;
const BUCKET_HALF_HEIGHT: f32 = 8.0;

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
            speed: num!(SPEED),
            walls,
        }
    }

    fn calculate_walls(
        position: Coordinates,
    ) -> [(Coordinates, Coordinates); 2] {
        [
            // Left wall
            (
                vec2(position.x, position.y),
                vec2(position.x, position.y + num!(16)),
            ),
            // Right wall
            (
                vec2(position.x + num!(32), position.y),
                vec2(position.x + num!(32), position.y + num!(16)),
            ),
        ]
    }

    pub fn update(&mut self) {
        self.position.x += self.direction * self.speed;

        if self.position.x <= num!(MIN_X) {
            self.direction = num!(1.0);
        } else if self.position.x >= num!(MAX_X) {
            self.direction = num!(-1.0);
        }

        // Update walls when position changes
        self.walls = Self::calculate_walls(self.position);
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame) {
        self.sprite.set_pos(self.position.round()).show(frame);
    }

    pub fn is_in_bucket(&self, position: Coordinates) -> bool {
        self.position.x < position.x
            && position.x < self.position.x + 32
            && self.position.y < position.y
            && position.y < self.position.y + 16
    }
}
