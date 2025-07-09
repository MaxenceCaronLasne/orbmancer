use crate::{Coordinates, Fixed};
use agb::display::object::{AffineMatrixObject, AffineMode, ObjectAffine};
use agb::display::{AffineMatrix, GraphicsFrame};
use agb::{
    fixnum::{num, vec2},
    include_aseprite,
};

const MAX_INPUT_VELOCITY: f32 = 100.0;
const VELOCITY_CHANGE_RATE: f32 = 120.0;

include_aseprite!(
    mod sprites,
    "assets/ball_direction.aseprite"
);

pub struct Launcher {
    position: Coordinates,
    velocity: Fixed,
    sprite: ObjectAffine,
    angle: Fixed,
}

impl Launcher {
    pub fn new(position: Coordinates) -> Self {
        let affine_matrix: AffineMatrix = AffineMatrix::from_rotation(num!(0));
        let affine_matrix_instance = AffineMatrixObject::new(affine_matrix);

        Self {
            position,
            velocity: num!(0),
            sprite: ObjectAffine::new(
                sprites::SEGMENT.sprite(0),
                affine_matrix_instance,
                AffineMode::Affine,
            ),
            angle: Self::angle(num!(0)),
        }
    }

    pub fn velocity(&self) -> Fixed {
        self.velocity
    }

    pub fn turn_left(&mut self, delta: Fixed) {
        self.velocity -= num!(VELOCITY_CHANGE_RATE) * delta;
        self.velocity = self
            .velocity
            .clamp(num!(-MAX_INPUT_VELOCITY), num!(MAX_INPUT_VELOCITY));
        self.angle = Self::angle(self.velocity);
    }

    pub fn turn_right(&mut self, delta: Fixed) {
        self.velocity += num!(VELOCITY_CHANGE_RATE) * delta;
        self.velocity = self
            .velocity
            .clamp(num!(-MAX_INPUT_VELOCITY), num!(MAX_INPUT_VELOCITY));
        self.angle = Self::angle(self.velocity);
    }

    fn angle(horizontal_velocity: Fixed) -> Fixed {
        let normalized_velocity =
            (horizontal_velocity / num!(100.0)).clamp(num!(-1.0), num!(1.0));

        normalized_velocity * num!(0.25) + num!(0.75)
    }

    pub fn show(&mut self, frame: &mut GraphicsFrame) {
        let affine_matrix: AffineMatrix =
            AffineMatrix::from_rotation(self.angle);
        let affine_matrix_instance = AffineMatrixObject::new(affine_matrix);

        self.sprite
            .set_affine_matrix(affine_matrix_instance)
            .set_pos(
                (self.position + vec2(num!(4), num!(4))
                    - vec2(num!(16), num!(16)))
                .round(),
            )
            .show(frame);
    }
}
