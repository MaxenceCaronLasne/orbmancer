use crate::Coordinates;
use agb::fixnum::{num, vec2};

pub struct GameConfig;

impl GameConfig {
    // Core Game Settings
    pub const DELTA_TIME: f32 = 1.0 / 60.0;
    pub const TARGET_SCORE: i32 = 1000;

    // Screen Boundaries
    pub const SCREEN_BOTTOM: f32 = 168.0;
    pub const WALL_LEFT: i32 = 3 * 8 + 1;
    pub const WALL_RIGHT: i32 = Self::WALL_LEFT + 160 - 8 - 1;

    // Starting Positions
    pub const BALL_START_X: f32 = 100.0;
    pub const BALL_START_Y: f32 = 11.0;
    pub const BUCKET_START_X: f32 = 80.0;
    pub const BUCKET_START_Y: f32 = 140.0;
    pub const POWER_GAUGE_Y: f32 = 3.0;

    // Physics Configuration
    pub const PHYSICS_GRAVITY: i32 = 200;
    pub const PHYSICS_REPULSION_STRENGTH: i32 = 3000;
    pub const PHYSICS_UPPER_WALL: i32 = 20;
    pub const PHYSICS_LOWER_WALL_PEGS: i32 = 130;
    pub const PHYSICS_LOWER_WALL_BALL: i32 = 180;
    pub const PHYSICS_OBJECT_RADIUS_BALL: i32 = 1;
    pub const PHYSICS_OBJECT_RADIUS_PEG: i32 = 4;

    // Entity Radii
    pub const BALL_RADIUS: i32 = 4;
    pub const PEG_RADIUS: i32 = 3;
    pub const BALL_ROTATION_SPEED: f32 = 0.005;

    // Peg Force Radii
    pub const PEG_FORCE_RADII: [f32; 4] = [5.0, 10.0, 15.0, 20.0];

    // Launcher Controls
    pub const LAUNCHER_MAX_INPUT_VELOCITY: f32 = 100.0;
    pub const LAUNCHER_VELOCITY_CHANGE_RATE: f32 = 120.0;
    pub const LAUNCHER_MIN_POWER: f32 = 0.2;
    pub const LAUNCHER_MAX_POWER: f32 = 3.0;
    pub const LAUNCHER_POWER_CHARGE_RATE: f32 = 3.0;

    // Bucket Movement
    pub const BUCKET_SPEED: f32 = 1.0;
    pub const BUCKET_WIDTH: i32 = 32;
    pub const BUCKET_HEIGHT: i32 = 16;
    pub const BUCKET_WALL_OFFSET_LEFT: i32 = 3;
    pub const BUCKET_WALL_OFFSET_RIGHT: i32 = 28;

    // Peg Generation
    pub const MAX_PEGS_TO_SPAWN: usize = 50;
    pub const PEG_SPAWN_SCREEN_HEIGHT: i32 = 120;
    pub const PEG_SPAWN_MIN_Y: i32 = 30;
    pub const PEG_SPAWN_VELOCITY_RANGE: i32 = 100;

    // UI Constants
    pub const JAUGE_PADDING: i32 = 3;
    pub const JAUGE_SPRITE_WIDTH: i32 = 32;
    pub const COUNTER_DIGIT_SPACING: f32 = 4.0;
    pub const COUNTER_POSITION_OFFSET: f32 = 2.0;
    pub const TEXT_LETTER_GROUP_SIZE: i32 = 16;
    pub const INVENTORY_ITEM_SPACING: i32 = 12;

    // Visual Effects
    pub const SHAKE_DURATION: u32 = 10;
    pub const SHAKE_INTENSITY: i32 = 3;
    pub const FLASH_DURATION: u32 = 3;

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
