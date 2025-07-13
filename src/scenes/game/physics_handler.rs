use crate::{error::Error, physics::{Physics, PhysicsConfig}, Fixed, Force, Coordinates};
use agb::{fixnum::num, rng::RandomNumberGenerator};
use alloc::vec::Vec;
use super::{ball, config::GameConfig, peg::{self, Pegs}};

pub struct PhysicsHandler;

const PEG_CONFIG: PhysicsConfig = PhysicsConfig {
    left_wall: GameConfig::WALL_LEFT,
    up_wall: 10,
    right_wall: GameConfig::WALL_RIGHT,
    down_wall: 130,
    moving_radius: peg::RADIUS,
    static_radius: peg::RADIUS,
    gravity: 0, // Pegs don't use gravity
    repulsion_strength: 3000,
    object_radius: 4,
};

const BALL_CONFIG: PhysicsConfig = PhysicsConfig {
    left_wall: GameConfig::WALL_LEFT,
    up_wall: 0,
    right_wall: GameConfig::WALL_RIGHT,
    down_wall: 180,
    moving_radius: ball::RADIUS,
    static_radius: peg::RADIUS,
    gravity: 200,
    repulsion_strength: 0, // Ball doesn't use repulsion
    object_radius: 1,
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
                    x if x >= 0 => x % 100,
                    x => x % -100,
                };
                let velo_y = match rng.next_i32() {
                    y if y >= 0 => y % 100,
                    y => y % -100,
                };
                pegs.velocities[i] = Force::new(Fixed::new(velo_x), Fixed::new(velo_y));

                return Ok(true);
            }
        }
        Ok(false)
    }


    pub fn hide_non_collidable_pegs<const MAX_PEGS: usize>(pegs: &mut Pegs<MAX_PEGS>) {
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
