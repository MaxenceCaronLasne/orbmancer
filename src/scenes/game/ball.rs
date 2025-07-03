use crate::Fixed;
use crate::{Coordinates, Force};
use agb::display::object::{AffineMatrixObject, AffineMode, ObjectAffine};
use agb::display::{AffineMatrix, GraphicsFrame};
use agb::{
    fixnum::{num, vec2},
    include_aseprite,
};

pub const RADIUS: i32 = 4;

include_aseprite!(
    mod sprites,
    "assets/ball.aseprite"
);

pub struct Ball {
    pub position: Coordinates,
    pub velocity: Force,
    sprite: ObjectAffine,
    sprite_rotation: Fixed,
}

impl Ball {
    pub fn new(position: Coordinates) -> Self {
        let affine_matrix: AffineMatrix = AffineMatrix::from_rotation(num!(0));
        let affine_matrix_instance = AffineMatrixObject::new(affine_matrix);
        Self {
            position,
            velocity: vec2(num!(0), num!(0)),
            sprite: ObjectAffine::new(
                sprites::BALL.sprite(0),
                affine_matrix_instance,
                AffineMode::Affine,
            ),
            sprite_rotation: num!(0),
        }
    }

    pub fn reset_sprite(&mut self) {
        self.sprite_rotation = num!(0);
    }

    pub fn update(&mut self) {
        self.sprite_rotation += num!(0.005);
    }

    pub fn show(self: &mut Ball, frame: &mut GraphicsFrame) {
        let affine_matrix: AffineMatrix =
            AffineMatrix::from_rotation(self.sprite_rotation);

        let affine_matrix_instance = AffineMatrixObject::new(affine_matrix);

        self.sprite
            .set_affine_matrix(affine_matrix_instance)
            .set_pos(self.position.round())
            .show(frame);
    }
}
