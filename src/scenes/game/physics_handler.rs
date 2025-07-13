use crate::{error::Error, physics::Physics, Fixed, Force, Coordinates};
use agb::{fixnum::num, rng::RandomNumberGenerator};
use alloc::vec::Vec;
use super::{ball, config::GameConfig, peg::{self, Pegs}};

pub struct PhysicsHandler;

impl PhysicsHandler {
    pub fn update_pegs<const MAX_PEGS: usize>(
        physics: &mut Physics<MAX_PEGS>,
        pegs: &mut Pegs<MAX_PEGS>,
        rng: &mut agb::rng::RandomNumberGenerator,
    ) -> Result<(), Error> {
        crate::bench::start("PEG_UPDATE");
        
        Self::update_green_peg_generation(pegs, physics, rng)?;
        
        let result = physics.move_from_fields::<
            3000, 10, { GameConfig::WALL_LEFT }, 10, { GameConfig::WALL_RIGHT }, 110, 15
        >(
            &mut pegs.positions,
            &mut pegs.velocities,
            &pegs.collidable,
            &pegs.force_radius_squared,
            num!(GameConfig::DELTA_TIME),
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
        let result = physics.move_and_collide::<
            { ball::RADIUS }, { peg::RADIUS }, 200, 
            { GameConfig::WALL_LEFT }, 0, { GameConfig::WALL_RIGHT }, 180
        >(
            ball_position,
            ball_velocity,
            &pegs.positions,
            &pegs.collidable,
            num!(GameConfig::DELTA_TIME),
            bucket_walls,
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
                let offset_x = (rng.next_i32() % 10) - 5;
                let offset_y = (rng.next_i32() % 10) - 5;
                let offset_position = spawn_position + Force::new(Fixed::new(offset_x), Fixed::new(offset_y));
                
                physics.force_move(i, offset_position, &mut pegs.positions)?;
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

    pub fn update_green_peg_generation<const MAX_PEGS: usize>(
        pegs: &mut Pegs<MAX_PEGS>,
        physics: &mut Physics<MAX_PEGS>,
        rng: &mut RandomNumberGenerator,
    ) -> Result<(), Error> {
        const GENERATION_INTERVAL: u32 = 10;
        
        let mut spawns_needed = Vec::new();
        
        for i in 0..MAX_PEGS {
            if let Some(timer) = &mut pegs.generation_timer[i] {
                *timer += 1;
                
                if *timer >= GENERATION_INTERVAL && pegs.pending_generations[i] > 0 {
                    if let Some(spawn_position) = pegs.generation_spawn_position[i] {
                        spawns_needed.push((i, spawn_position));
                    }
                }
            }
        }
        
        for (i, spawn_position) in spawns_needed {
            if Self::spawn_single_peg_from_green(pegs, physics, spawn_position, rng)? {
                pegs.pending_generations[i] -= 1;
                if let Some(timer) = &mut pegs.generation_timer[i] {
                    *timer = 0;
                }
                
                if pegs.pending_generations[i] == 0 {
                    pegs.generation_timer[i] = None;
                    pegs.generation_spawn_position[i] = None;
                }
            }
        }
        
        Ok(())
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
