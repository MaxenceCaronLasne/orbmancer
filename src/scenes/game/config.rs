use crate::Coordinates;
use agb::fixnum::{num, vec2};

pub struct GameConfig;

impl GameConfig {
    pub const DELTA_TIME: f32 = 1.0 / 60.0;
    pub const BALL_START_X: f32 = 100.0;
    pub const BALL_START_Y: f32 = 11.0;
    pub const BUCKET_START_X: f32 = 80.0;
    pub const BUCKET_START_Y: f32 = 140.0;
    pub const SCREEN_BOTTOM: f32 = 168.0;
    pub const TARGET_SCORE: i32 = 100;
    pub const WALL_LEFT: i32 = 3 * 8 + 1;
    pub const WALL_RIGHT: i32 = Self::WALL_LEFT + 160 - 8 - 1;
    pub const SHAKE_DURATION: u32 = 10;
    pub const SHAKE_INTENSITY: i32 = 3;
    pub const FLASH_DURATION: u32 = 3;
    pub const POWER_GAUGE_Y: f32 = 3.0;

    pub fn ball_start_pos() -> Coordinates {
        vec2(num!(Self::BALL_START_X), num!(Self::BALL_START_Y))
    }

    pub fn bucket_start_pos() -> Coordinates {
        vec2(num!(Self::BUCKET_START_X), num!(Self::BUCKET_START_Y))
    }

    pub fn power_gauge_pos() -> Coordinates {
        vec2(num!(Self::BALL_START_X - 24.0), num!(Self::POWER_GAUGE_Y))
    }
}
