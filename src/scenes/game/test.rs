// This module is only compiled during tests
// rust-analyzer should respect this and not show errors during normal editing

#![cfg(test)]
// Disable clippy warnings for test-only code
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::Fixed;
use crate::physics::Physics;
use crate::scenes::game::peg::{FORCE_RADII, Kind};
use agb::Gba;
use agb::fixnum::{num, vec2};
use agb::rng::RandomNumberGenerator;

#[test_case]
pub fn test_physics_performance(gba: &mut Gba) {
    let mut timers = gba.timers.timers();
    crate::bench::init(&mut timers);

    const PEG_COUNT: usize = 50;
    const MAX_PEGS: usize = 50;

    let mut rng = RandomNumberGenerator::new();
    let screen_width = 140;
    let screen_height = 120;
    let min_x = 20;
    let min_y = 30;

    let mut positions = [vec2(num!(0), num!(0)); MAX_PEGS];
    let mut force_radius_squared = [num!(20); MAX_PEGS];
    let mut showable = [false; MAX_PEGS];
    let mut collidable = [false; MAX_PEGS];
    let mut kind = [Kind::Blue; MAX_PEGS];
    let mut velocities = [vec2(num!(0), num!(0)); MAX_PEGS];

    crate::bench::start("PEG_GENERATION");
    for i in 0..PEG_COUNT {
        let x = min_x + (rng.next_i32().abs() % (screen_width - min_x));
        let y = min_y + (rng.next_i32().abs() % (screen_height - min_y));

        let force_radius_index =
            (rng.next_i32().abs() % FORCE_RADII.len() as i32) as usize;
        let force_radius = Fixed::new(FORCE_RADII[force_radius_index] as i32);

        positions[i] = vec2(Fixed::new(x), Fixed::new(y));
        force_radius_squared[i] = force_radius * force_radius;
        showable[i] = true;
        collidable[i] = true;

        kind[i] = if rng.next_i32() > 0 {
            Kind::Blue
        } else {
            Kind::Red
        };
    }
    crate::bench::stop("PEG_GENERATION");

    crate::bench::start("PHYSICS_INIT");
    let mut physics = Physics::<MAX_PEGS>::new(&positions, &collidable)
        .expect("Physics init failed");
    crate::bench::stop("PHYSICS_INIT");

    let simulation_frames = 100;
    let delta_time = num!(1.0 / 60.0);

    crate::bench::start("PHYSICS_SIMULATION");
    for frame in 0..simulation_frames {
        crate::bench::start("PHYSICS_FRAME");

        physics
            .move_from_fields::<3000, 10, 10, 10, 150, 110, 15>(
                &mut positions,
                &mut velocities,
                &collidable,
                &force_radius_squared,
                delta_time,
            )
            .expect("Physics frame failed");

        crate::bench::stop("PHYSICS_FRAME");
    }
    crate::bench::stop("PHYSICS_SIMULATION");

    let mut moved_pegs = 0;
    for i in 0..PEG_COUNT {
        if velocities[i].x != num!(0) || velocities[i].y != num!(0) {
            moved_pegs += 1;
        }
    }

    crate::bench::log();
}

#[test_case]
pub fn test_physics_collision_accuracy(gba: &mut Gba) {
    let mut timers = gba.timers.timers();
    crate::bench::init(&mut timers);

    const MAX_PEGS: usize = 10;

    let mut positions = [vec2(num!(0), num!(0)); MAX_PEGS];
    let mut velocities = [vec2(num!(0), num!(0)); MAX_PEGS];
    let mut collidable = [false; MAX_PEGS];
    let force_radius_squared = [num!(25); MAX_PEGS]; // radius = 5

    positions[0] = vec2(num!(50), num!(50));
    positions[1] = vec2(num!(60), num!(50)); // 10 units apart
    positions[2] = vec2(num!(50), num!(60)); // 10 units apart
    collidable[0] = true;
    collidable[1] = true;
    collidable[2] = true;

    velocities[0] = vec2(num!(1), num!(0));
    velocities[1] = vec2(num!(-1), num!(0));

    crate::bench::start("COLLISION_TEST");
    let mut physics = Physics::<MAX_PEGS>::new(&positions, &collidable)
        .expect("Physics init failed");

    for _step in 0..10 {
        physics
            .move_from_fields::<3000, 10, 10, 10, 150, 110, 15>(
                &mut positions,
                &mut velocities,
                &collidable,
                &force_radius_squared,
                num!(1.0 / 60.0),
            )
            .expect("Physics step failed");
    }
    crate::bench::stop("COLLISION_TEST");

    let initial_pos = vec2(num!(50), num!(50));
    let pos_changed = positions[0] != initial_pos;

    crate::bench::log();
}

