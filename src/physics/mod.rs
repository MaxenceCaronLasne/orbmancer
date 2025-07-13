use crate::error::Error;
use crate::{Coordinates, Fixed, Force};
use agb::fixnum::{num, vec2};
use alloc::vec::Vec;

mod grid;

#[derive(Clone, Copy)]
pub struct PhysicsConfig {
    pub left_wall: i32,
    pub up_wall: i32,
    pub right_wall: i32,
    pub down_wall: i32,
    pub moving_radius: i32,
    pub static_radius: i32,
    pub gravity: i32,
    pub repulsion_strength: i32,
    pub object_radius: i32,
}

const WALL_BOUNCE_DAMPING: f32 = 0.9;
const BOUNCE_DAMPING: f32 = 0.9;
const ESCAPE_FORCE: i32 = 15;
const WALL_SLIDE_DAMPING: f32 = 0.8;
const MIN_DISTANCE_THRESHOLD: f32 = 0.001;
const VELOCITY_DAMPING: f32 = 0.98;
const MIN_BOUNCE_SPEED_SQUARED: i32 = 625;
const HALF_MIN_VELOCITY: i32 = 12;
const RANDOMNESS_INTERVAL: usize = 4;
const RAND_X_MULTIPLIER: usize = 31;
const RAND_Y_MULTIPLIER: usize = 37;
const RAND_X_OFFSET: usize = 17;
const RAND_Y_OFFSET: usize = 23;
const RAND_MODULO: usize = 10;
const RAND_CENTER_OFFSET: i32 = 5;
const MIN_DISTANCE_CHECK: f32 = 0.1;
const OVERLAP_ADJUSTMENT: f32 = 0.5;
const VELOCITY_CLAMP_LIMIT: i32 = 50;
const NEIGHBOR_SEARCH_RADIUS: usize = 1;
const FORCE_SEARCH_RADIUS: usize = 5;

pub struct Physics<const N: usize> {
    neighbors: grid::Grid2D,
    touched: Vec<usize>,
    frame_counter: usize,
}

impl<const N: usize> Physics<N> {
    pub fn new(
        positions: &[Coordinates; N],
        collidable: &[bool; N],
    ) -> Result<Self, Error> {
        Ok(Self {
            neighbors: grid::Grid2D::new(positions, collidable)?,
            touched: Vec::new(),
            frame_counter: 0,
        })
    }

    fn hit_horizontal_walls(
        position: &mut Coordinates,
        velocity: &mut Force,
        radius: Fixed,
        config: &PhysicsConfig,
    ) -> bool {
        if position.x < Fixed::new(config.left_wall) + radius {
            position.x = Fixed::new(config.left_wall) + radius;
            velocity.x = -velocity.x * num!(WALL_BOUNCE_DAMPING) + num!(ESCAPE_FORCE);
            true
        } else if position.x > Fixed::new(config.right_wall) - radius {
            position.x = Fixed::new(config.right_wall) - radius;
            velocity.x = -velocity.x * num!(WALL_BOUNCE_DAMPING) - num!(ESCAPE_FORCE);
            true
        } else {
            false
        }
    }

    fn hit_vertical_walls(
        position: &mut Coordinates,
        velocity: &mut Force,
        radius: Fixed,
        config: &PhysicsConfig,
    ) -> bool {
        if position.y < Fixed::new(config.up_wall) + radius {
            position.y = Fixed::new(config.up_wall) + radius;
            velocity.y = -velocity.y * num!(WALL_BOUNCE_DAMPING) + num!(ESCAPE_FORCE);
            true
        } else if position.y > Fixed::new(config.down_wall) - radius {
            position.y = Fixed::new(config.down_wall) - radius;
            velocity.y = -velocity.y * num!(WALL_BOUNCE_DAMPING) - num!(ESCAPE_FORCE);
            true
        } else {
            false
        }
    }

    fn apply_corner_escape(
        position: Coordinates,
        velocity: &mut Force,
        hit_x: bool,
        hit_y: bool,
        config: &PhysicsConfig,
    ) {
        if hit_x && hit_y {
            let center_x = Fixed::new((config.left_wall + config.right_wall) / 2);
            let center_y = Fixed::new((config.up_wall + config.down_wall) / 2);

            if position.x < center_x {
                velocity.x += num!(ESCAPE_FORCE);
            } else {
                velocity.x -= num!(ESCAPE_FORCE);
            }

            if position.y < center_y {
                velocity.y += num!(ESCAPE_FORCE);
            } else {
                velocity.y -= num!(ESCAPE_FORCE);
            }
        }
    }

    fn apply_wall_slide_damping(
        velocity: &mut Force,
        hit_x: bool,
        hit_y: bool,
    ) {
        if hit_x {
            velocity.y *= num!(WALL_SLIDE_DAMPING);
        }
        if hit_y {
            velocity.x *= num!(WALL_SLIDE_DAMPING);
        }
    }

    fn move_and_collide_with_walls(
        mut position: Coordinates,
        mut velocity: Force,
        radius: Fixed,
        config: &PhysicsConfig,
    ) -> (Coordinates, Force) {
        let hit_x = Self::hit_horizontal_walls(&mut position, &mut velocity, radius, config);
        let hit_y = Self::hit_vertical_walls(&mut position, &mut velocity, radius, config);

        Self::apply_corner_escape(position, &mut velocity, hit_x, hit_y, config);
        Self::apply_wall_slide_damping(&mut velocity, hit_x, hit_y);

        (position, velocity)
    }

    fn find_closest_point_on_segment(
        position: Coordinates,
        wall_start: Coordinates,
        wall_end: Coordinates,
    ) -> Option<Coordinates> {
        let wall_vector = wall_end - wall_start;
        let wall_length_squared = wall_vector.magnitude_squared();

        if wall_length_squared <= num!(MIN_DISTANCE_THRESHOLD) {
            return None;
        }

        let ball_to_start = position - wall_start;
        let t = (ball_to_start.dot(wall_vector) / wall_length_squared)
            .clamp(num!(0), num!(1));

        Some(wall_start + wall_vector * t)
    }

    fn collide_with_line_segment(
        mut position: Coordinates,
        mut velocity: Force,
        radius: Fixed,
        wall_start: Coordinates,
        wall_end: Coordinates,
    ) -> (Coordinates, Force) {
        let Some(closest_point) =
            Self::find_closest_point_on_segment(position, wall_start, wall_end)
        else {
            return (position, velocity);
        };

        let distance_vector = position - closest_point;
        let distance_squared = distance_vector.magnitude_squared();
        let radius_squared = radius * radius;

        if distance_squared < radius_squared {
            let distance = distance_squared.sqrt();
            if distance > num!(MIN_DISTANCE_THRESHOLD) {
                let normal = distance_vector / distance;
                position = closest_point + normal * radius;

                let velocity_along_normal = velocity.dot(normal);
                velocity -= -normal
                    * (velocity_along_normal * num!(2))
                    * num!(WALL_BOUNCE_DAMPING);
            }
        }

        (position, velocity)
    }

    fn detect_static_collisions(
        &mut self,
        position: Coordinates,
        positions: &[Coordinates; N],
        collidable: &[bool; N],
        config: &PhysicsConfig,
    ) -> Result<Vec<(usize, Coordinates, Fixed, Fixed)>, Error> {
        self.touched.clear();
        let neighbors = self
            .neighbors
            .get_neighbors(position, NEIGHBOR_SEARCH_RADIUS)?;
        let mut collisions = Vec::new();
        let collision_distance = Fixed::new(config.moving_radius + config.static_radius);
        let collision_distance_squared =
            collision_distance * collision_distance;

        for &i in neighbors {
            if !collidable[i as usize] {
                continue;
            }

            let distance_vector = position - positions[i as usize];
            let distance_squared = distance_vector.magnitude_squared();

            if distance_squared < collision_distance_squared {
                self.touched.push(i as usize);
                let distance = distance_squared.sqrt();
                if distance > num!(MIN_DISTANCE_THRESHOLD) {
                    collisions.push((
                        i as usize,
                        distance_vector,
                        distance,
                        collision_distance,
                    ));
                }
            }
        }

        Ok(collisions)
    }

    fn resolve_static_collision(
        position: &mut Coordinates,
        velocity: &mut Force,
        distance_vector: Coordinates,
        distance: Fixed,
        collision_distance: Fixed,
    ) {
        let normal = distance_vector / distance;
        let velocity_along_normal = velocity.dot(normal);
        *velocity -= normal * (velocity_along_normal * num!(2));
        *velocity *= num!(BOUNCE_DAMPING);
        let overlap = collision_distance - distance;
        *position += normal * overlap * num!(OVERLAP_ADJUSTMENT);
    }

    fn move_and_collide_with_statics(
        &mut self,
        mut position: Coordinates,
        mut velocity: Force,
        positions: &[Coordinates; N],
        collidable: &[bool; N],
        config: &PhysicsConfig,
    ) -> Result<(Coordinates, Force, &[usize]), Error> {
        let collisions = self.detect_static_collisions(position, positions, collidable, config)?;

        for (_, distance_vector, distance, collision_distance) in collisions {
            Self::resolve_static_collision(
                &mut position,
                &mut velocity,
                distance_vector,
                distance,
                collision_distance,
            );
        }

        Ok((position, velocity, &self.touched))
    }

    pub fn move_and_collide(
        &mut self,
        mut position: Coordinates,
        mut velocity: Force,
        positions: &[Coordinates; N],
        collidable: &[bool; N],
        delta: Fixed,
        walls: &[(Coordinates, Coordinates)],
        config: &PhysicsConfig,
    ) -> Result<(Coordinates, Force, &[usize]), Error> {
        velocity += Force::new(num!(0), Fixed::new(config.gravity)) * delta;
        position += velocity * delta;

        (position, velocity) = Self::move_and_collide_with_walls(position, velocity, num!(1), config);

        for &(wall_start, wall_end) in walls {
            (position, velocity) = Self::collide_with_line_segment(
                position,
                velocity,
                Fixed::new(config.moving_radius),
                wall_start,
                wall_end,
            );
        }

        let (position, velocity, touched) = self.move_and_collide_with_statics(
            position, velocity, positions, collidable, config,
        )?;

        Ok((position, velocity, touched))
    }

    fn apply_repulsion_forces(
        &mut self,
        index: usize,
        position: Coordinates,
        force_radius: Fixed,
        positions: &[Coordinates; N],
        velocities: &mut [Force; N],
        delta: Fixed,
        config: &PhysicsConfig,
    ) -> Result<(), Error> {
        let neighbors = self
            .neighbors
            .get_neighbors(position, FORCE_SEARCH_RADIUS)?;

        for &neighbor_index in neighbors {
            let neighbor_index = neighbor_index as usize;
            if neighbor_index == index {
                continue;
            }

            let distance_vector = position - positions[neighbor_index];
            let distance_squared = distance_vector.magnitude_squared();

            if distance_squared < num!(MIN_DISTANCE_CHECK)
                || distance_squared >= force_radius
            {
                continue;
            }

            let normal = distance_vector / distance_squared;
            let force_magnitude =
                Fixed::new(config.repulsion_strength * config.repulsion_strength)
                    / distance_squared;
            let repulsion_force = normal * force_magnitude;
            let new_velocity = velocities[index] + repulsion_force * delta;

            velocities[index] = vec2(
                new_velocity.x.clamp(
                    num!(-VELOCITY_CLAMP_LIMIT),
                    num!(VELOCITY_CLAMP_LIMIT),
                ),
                new_velocity.y.clamp(
                    num!(-VELOCITY_CLAMP_LIMIT),
                    num!(VELOCITY_CLAMP_LIMIT),
                ),
            );
        }

        Ok(())
    }

    fn update_object_movement(
        index: usize,
        positions: &mut [Coordinates; N],
        velocities: &mut [Force; N],
        frame_counter: usize,
        config: &PhysicsConfig,
    ) {
        velocities[index] *= num!(VELOCITY_DAMPING);

        let old_velocity = velocities[index];
        (positions[index], velocities[index]) = Self::move_and_collide_with_walls(
            positions[index],
            velocities[index],
            Fixed::new(config.object_radius),
            config,
        );

        if velocities[index] != old_velocity {
            Self::ensure_minimum_velocity(
                &mut velocities[index],
                index,
                frame_counter,
            );
        }
    }

    fn boost_low_velocity_components(velocity: &mut Force) {
        if velocity.x.abs() < num!(HALF_MIN_VELOCITY) {
            velocity.x = if velocity.x >= num!(0) {
                num!(HALF_MIN_VELOCITY)
            } else {
                num!(-HALF_MIN_VELOCITY)
            };
        }
        if velocity.y.abs() < num!(HALF_MIN_VELOCITY) {
            velocity.y = if velocity.y >= num!(0) {
                num!(HALF_MIN_VELOCITY)
            } else {
                num!(-HALF_MIN_VELOCITY)
            };
        }
    }

    fn add_randomness(
        velocity: &mut Force,
        index: usize,
        frame_counter: usize,
    ) {
        if (frame_counter + index) % RANDOMNESS_INTERVAL == 0 {
            let rand_x = ((index * RAND_X_MULTIPLIER
                + frame_counter * RAND_X_OFFSET)
                % RAND_MODULO) as i32
                - RAND_CENTER_OFFSET;
            let rand_y = ((index * RAND_Y_MULTIPLIER
                + frame_counter * RAND_Y_OFFSET)
                % RAND_MODULO) as i32
                - RAND_CENTER_OFFSET;

            velocity.x += Fixed::new(rand_x);
            velocity.y += Fixed::new(rand_y);
        }
    }

    fn ensure_minimum_velocity(
        velocity: &mut Force,
        index: usize,
        frame_counter: usize,
    ) {
        let speed_squared = velocity.magnitude_squared();

        if speed_squared < num!(MIN_BOUNCE_SPEED_SQUARED)
            && speed_squared > num!(0)
        {
            Self::boost_low_velocity_components(velocity);
        }

        Self::add_randomness(velocity, index, frame_counter);
    }

    pub fn move_from_fields<const PARTIAL: usize>(
        &mut self,
        positions: &mut [Coordinates; N],
        velocities: &mut [Force; N],
        collidable: &[bool; N],
        force_radius_squared: &[Fixed; N],
        delta: Fixed,
        config: &PhysicsConfig,
    ) -> Result<(), Error> {
        self.frame_counter = self.frame_counter.wrapping_add(1);

        for i in 0..N {
            if !collidable[i] {
                continue;
            }

            let position = positions[i];
            let force = force_radius_squared[i];
            let velocity = velocities[i];

            if i % PARTIAL == self.frame_counter % PARTIAL {
                self.apply_repulsion_forces(
                    i,
                    position,
                    force,
                    positions,
                    velocities,
                    delta * num!(PARTIAL),
                    config,
                )?;
            }

            Self::update_object_movement(i, positions, velocities, self.frame_counter, config);

            let new_pos =
                grid::clamp_position_to_grid(position + velocity * delta);

            positions[i] = new_pos;

            if new_pos != position {
                self.neighbors.update(i, position, new_pos)?;
            }
        }

        self.resolve_object_collisions(positions, velocities, collidable, config)?;

        for i in 0..N {
            if !collidable[i] {
                continue;
            }

            let old_position = positions[i];
            let new_pos = grid::clamp_position_to_grid(positions[i]);

            if new_pos != old_position {
                positions[i] = new_pos;
                self.neighbors.update(i, old_position, new_pos)?;
            }
        }

        Ok(())
    }

    fn resolve_object_collisions(
        &mut self,
        positions: &mut [Coordinates; N],
        velocities: &mut [Force; N],
        collidable: &[bool; N],
        config: &PhysicsConfig,
    ) -> Result<(), Error> {
        let collision_distance = Fixed::new(config.object_radius * 2);
        let collision_distance_squared =
            collision_distance * collision_distance;

        let mut position_updates = Vec::new();

        for i in 0..N {
            if !collidable[i] {
                continue;
            }

            let neighbors = self
                .neighbors
                .get_neighbors(positions[i], NEIGHBOR_SEARCH_RADIUS)?;

            for &j in neighbors {
                let j = j as usize;
                if j <= i || !collidable[j] {
                    continue;
                }

                let distance_vector = positions[i] - positions[j];
                let distance_squared = distance_vector.magnitude_squared();

                if distance_squared < collision_distance_squared
                    && distance_squared > num!(MIN_DISTANCE_THRESHOLD)
                {
                    let distance = distance_squared.sqrt();
                    let normal = distance_vector / distance;
                    let overlap = collision_distance - distance;

                    let separation =
                        normal * overlap * num!(OVERLAP_ADJUSTMENT);
                    position_updates.push((i, positions[i] + separation));
                    position_updates.push((j, positions[j] - separation));

                    let relative_velocity = velocities[i] - velocities[j];
                    let velocity_along_normal = relative_velocity.dot(normal);

                    if velocity_along_normal < num!(0) {
                        let impulse = normal
                            * velocity_along_normal
                            * num!(BOUNCE_DAMPING);
                        velocities[i] -= impulse;
                        velocities[j] += impulse;
                    }
                }
            }
        }

        for (index, new_position) in position_updates {
            let old_position = positions[index];
            positions[index] = new_position;
            if old_position != new_position {
                self.neighbors.update(index, old_position, new_position)?;
            }
        }

        Ok(())
    }

    pub fn force_move(
        &mut self,
        index: usize,
        at: Coordinates,
        positions: &mut [Coordinates; N],
    ) -> Result<(), Error> {
        let old = positions[index];
        positions[index] = at;
        self.neighbors.update(index, old, at)?;

        Ok(())
    }
}
