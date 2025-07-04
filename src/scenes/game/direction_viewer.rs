use crate::{Coordinates, Fixed};
use agb::display::object::{AffineMatrixObject, AffineMode, ObjectAffine};
use agb::display::{AffineMatrix, GraphicsFrame};
use agb::{
    fixnum::{num, vec2},
    include_aseprite,
};

include_aseprite!(
    mod sprites,
    "assets/ball_direction.aseprite"
);

pub struct DirectionViewer {
    pub position: Coordinates,
    sprite: ObjectAffine,
    angle: Fixed,
}

impl DirectionViewer {
    pub fn new(position: Coordinates) -> Self {
        let affine_matrix: AffineMatrix = AffineMatrix::from_rotation(num!(0));
        let affine_matrix_instance = AffineMatrixObject::new(affine_matrix);
        Self {
            position,
            sprite: ObjectAffine::new(
                sprites::SEGMENT.sprite(0),
                affine_matrix_instance,
                AffineMode::Affine,
            ),
            angle: num!(0),
        }
    }

    pub fn update_direction(&mut self, horizontal_velocity: Fixed) {
        let normalized_velocity =
            (horizontal_velocity / num!(100.0)).clamp(num!(-1.0), num!(1.0));
        self.angle = normalized_velocity * num!(0.25) + num!(0.75);
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
