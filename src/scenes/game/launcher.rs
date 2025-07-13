use crate::{Coordinates, Fixed};
use agb::display::object::{AffineMatrixObject, AffineMode, ObjectAffine};
use agb::display::{AffineMatrix, GraphicsFrame};
use agb::{
    fixnum::{num, vec2},
    include_aseprite,
};

const MAX_INPUT_VELOCITY: f32 = 100.0;
const VELOCITY_CHANGE_RATE: f32 = 120.0;
const MIN_POWER: f32 = 0.2;
const MAX_POWER: f32 = 3.0;
const POWER_CHARGE_RATE: f32 = 2.8;

include_aseprite!(
    mod sprites,
    "assets/ball_direction.aseprite"
);

pub struct Launcher {
    position: Coordinates,
    velocity: Fixed,
    sprite: ObjectAffine,
    angle: Fixed,
    power_charge: Fixed,
    is_charging: bool,
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
            power_charge: num!(MIN_POWER),
            is_charging: false,
        }
    }

    pub fn velocity(&self) -> Fixed {
        self.velocity
    }

    pub fn start_charging(&mut self) {
        self.is_charging = true;
        self.power_charge = num!(MIN_POWER);
    }

    pub fn charge_power(&mut self, delta: Fixed) {
        if self.is_charging {
            self.power_charge += num!(POWER_CHARGE_RATE) * delta;
            self.power_charge = self.power_charge.clamp(num!(MIN_POWER), num!(MAX_POWER));
        }
    }

    pub fn get_launch_power(&self) -> Fixed {
        self.power_charge
    }

    pub fn stop_charging(&mut self) -> Fixed {
        self.is_charging = false;
        let power = self.power_charge;
        self.power_charge = num!(MIN_POWER);
        power
    }

    pub fn is_charging(&self) -> bool {
        self.is_charging
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
