use crate::types::Fixed;
use agb::fixnum::num;

pub type ForceStrength = Fixed;
pub type DampingFactor = Fixed;

pub const GRAVITY_Y: f32 = 200.0;
pub const ZERO: f32 = 0.0;
pub const LEFT_WALL: f32 = 0.0;
pub const RIGHT_WALL: f32 = 160.0;
pub const SCREEN_BOTTOM: f32 = 180.0;
pub const WALL_BOUNCE_DAMPING: f32 = 0.9;
pub const PEG_BOUNCE_DAMPING: f32 = 0.9;

#[derive(Clone, Copy, Debug)]
pub struct PhysicsConfig {
    pub gravity_y: ForceStrength,
    pub peg_bounce_damping: DampingFactor,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity_y: num!(GRAVITY_Y),
            peg_bounce_damping: num!(PEG_BOUNCE_DAMPING),
        }
    }
}
