use super::{config::GameConfig, peg::Pegs};
use crate::{
    Coordinates, Fixed, Force,
    error::Error,
    physics::{Physics, PhysicsConfig},
};
use agb::{fixnum::num, rng::RandomNumberGenerator};
use alloc::vec::Vec;

pub struct PhysicsHandler;

const PEG_CONFIG: PhysicsConfig = PhysicsConfig {
    left_wall: GameConfig::WALL_LEFT,
    up_wall: GameConfig::PHYSICS_UPPER_WALL,
    right_wall: GameConfig::WALL_RIGHT,
    down_wall: GameConfig::PHYSICS_LOWER_WALL_PEGS,
    moving_radius: GameConfig::PEG_RADIUS,
    static_radius: GameConfig::PEG_RADIUS,
    gravity: 0, // Pegs don't use gravity
    repulsion_strength: GameConfig::PHYSICS_REPULSION_STRENGTH,
    object_radius: GameConfig::PHYSICS_OBJECT_RADIUS_PEG,
};

const BALL_CONFIG: PhysicsConfig = PhysicsConfig {
    left_wall: GameConfig::WALL_LEFT,
    up_wall: 0,
    right_wall: GameConfig::WALL_RIGHT,
    down_wall: GameConfig::PHYSICS_LOWER_WALL_BALL,
    moving_radius: GameConfig::BALL_RADIUS,
    static_radius: GameConfig::PEG_RADIUS,
    gravity: GameConfig::PHYSICS_GRAVITY,
    repulsion_strength: 0, // Ball doesn't use repulsion
    object_radius: GameConfig::PHYSICS_OBJECT_RADIUS_BALL,
};

impl PhysicsHandler {
    pub fn update_pegs<const MAX_PEGS: usize>(
        physics: &mut Physics<MAX_PEGS>,
        pegs: &mut Pegs<MAX_PEGS>,
    ) -> Result<(), Error> {
        crate::bench::start("PEG_UPDATE");
        let result = physics.move_from_fields::<15>(
            &mut pegs.positions,
            &mut pegs.velocities,
            &pegs.collidable,
            &pegs.force_radius_squared,
            num!(GameConfig::DELTA_TIME),
            &PEG_CONFIG,
        );
        crate::bench::stop("PEG_UPDATE");
        result
    }

    pub fn move_ball_and_detect_collisions<const MAX_PEGS: usize>(
        physics: &mut Physics<MAX_PEGS>,
        ball_position: Coordinates,
        ball_velocity: crate::Force,
        pegs: &Pegs<MAX_PEGS>,
        bucket_walls: &[(Coordinates, Coordinates); 2],
    ) -> Result<(Coordinates, crate::Force, Vec<usize>), Error> {
        crate::bench::start("UPDATE_BALL_TOP");
        let result = physics.move_and_collide(
            ball_position,
            ball_velocity,
            &pegs.positions,
            &pegs.collidable,
            num!(GameConfig::DELTA_TIME),
            bucket_walls,
            &BALL_CONFIG,
        )?;
        crate::bench::stop("UPDATE_BALL_TOP");
        Ok((result.0, result.1, result.2.to_vec()))
    }

    pub fn spawn_single_peg_from_green<const MAX_PEGS: usize>(
        pegs: &mut Pegs<MAX_PEGS>,
        physics: &mut Physics<MAX_PEGS>,
        spawn_position: Coordinates,
        rng: &mut RandomNumberGenerator,
    ) -> Result<bool, Error> {
        for i in 0..MAX_PEGS {
            if !pegs.showable[i] {
                physics.force_move(i, spawn_position, &mut pegs.positions)?;
                pegs.showable[i] = true;
                pegs.collidable[i] = true;

                let velo_x = match rng.next_i32() {
                    x if x >= 0 => x % GameConfig::PEG_SPAWN_VELOCITY_RANGE,
                    x => x % -GameConfig::PEG_SPAWN_VELOCITY_RANGE,
                };
                let velo_y = match rng.next_i32() {
                    y if y >= 0 => y % GameConfig::PEG_SPAWN_VELOCITY_RANGE,
                    y => y % -GameConfig::PEG_SPAWN_VELOCITY_RANGE,
                };
                pegs.velocities[i] =
                    Force::new(Fixed::new(velo_x), Fixed::new(velo_y));

                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn hide_non_collidable_pegs<const MAX_PEGS: usize>(
        pegs: &mut Pegs<MAX_PEGS>,
    ) {
        pegs.collidable
            .iter()
            .zip(pegs.showable.iter_mut())
            .for_each(|(&is_collidable, showable)| {
                if !is_collidable {
                    *showable = false;
                }
            });
    }
}
