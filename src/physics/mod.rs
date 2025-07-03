use crate::error::Error;
use crate::{Coordinates, Fixed, Force};
use agb::fixnum::{num, vec2};
use alloc::vec::Vec;

mod grid;

const WALL_BOUNCE_DAMPING: f32 = 0.9;
const BOUNCE_DAMPING: f32 = 0.9;

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

    fn move_and_collide_with_walls<
        const LEFT_WALL: i32,
        const UP_WALL: i32,
        const RIGHT_WALL: i32,
        const DOWN_WALL: i32,
    >(
        mut position: Coordinates,
        mut velocity: Force,
        radius: Fixed,
    ) -> (Coordinates, Force) {
        if position.x < num!(LEFT_WALL) + radius {
            position.x = num!(LEFT_WALL) + radius;
            velocity.x = -velocity.x * num!(WALL_BOUNCE_DAMPING);
        } else if position.x > num!(RIGHT_WALL) - radius {
            position.x = num!(RIGHT_WALL) - radius;
            velocity.x = -velocity.x * num!(WALL_BOUNCE_DAMPING);
        }

        if position.y < num!(UP_WALL) + radius {
            position.y = num!(UP_WALL) + radius;
            velocity.y = -velocity.y * num!(WALL_BOUNCE_DAMPING);
        } else if position.y > num!(DOWN_WALL) - radius {
            position.y = num!(DOWN_WALL) - radius;
            velocity.y = -velocity.y * num!(WALL_BOUNCE_DAMPING);
        }

        (position, velocity)
    }

    fn move_and_collide_with_given_wall(
        mut position: Coordinates,
        mut velocity: Force,
        radius: Fixed,
        wall_start: Coordinates,
        wall_end: Coordinates,
    ) -> (Coordinates, Force) {
        // 1. Find closest point on line segment to ball center
        let wall_vector = wall_end - wall_start;
        let ball_to_start = position - wall_start;

        let wall_length_squared = wall_vector.magnitude_squared();
        if wall_length_squared <= num!(0.001) {
            return (position, velocity); // Degenerate case
        }

        let t = (ball_to_start.dot(wall_vector) / wall_length_squared)
            .clamp(num!(0), num!(1));
        let closest_point = wall_start + wall_vector * t;

        // 2. Check if ball intersects the wall segment
        let distance_vector = position - closest_point;
        let distance_squared = distance_vector.magnitude_squared();
        let radius_squared = radius * radius;

        if distance_squared < radius_squared {
            // 3. Determine collision normal and side
            let distance = distance_squared.sqrt();
            if distance > num!(0.001) {
                let normal = distance_vector / distance;

                // Reposition ball outside the wall
                position = closest_point + normal * radius;

                // Apply velocity reflection along the normal
                let velocity_along_normal = velocity.dot(normal);
                velocity = velocity
                    - normal
                        * (velocity_along_normal * num!(2))
                        * num!(WALL_BOUNCE_DAMPING);
            }
        }

        (position, velocity)
    }

    fn move_and_collide_with_statics<
        const MOVING_RADIUS: i32,
        const STATIC_RADIUS: i32,
    >(
        &mut self,
        mut position: Coordinates,
        mut velocity: Force,
        positions: &[Coordinates; N],
        collidable: &[bool; N],
    ) -> Result<(Coordinates, Force, &[usize]), Error> {
        self.touched.clear();
        let neighbors = self.neighbors.get_neighbors(position, 1)?;

        for &i in neighbors {
            if !collidable[i as usize] {
                continue;
            }

            let distance_vector = position - positions[i as usize];
            let distance_squared = distance_vector.magnitude_squared();
            let collision_distance = Fixed::new(MOVING_RADIUS + STATIC_RADIUS);
            let collision_distance_squared =
                collision_distance * collision_distance;

            if distance_squared < collision_distance_squared {
                self.touched.push(i as usize);
                let distance = distance_squared.sqrt();
                if distance > num!(0.001) {
                    let inv_distance = num!(1) / distance;
                    let normal = distance_vector * inv_distance;
                    let velocity_along_normal = velocity.dot(normal);
                    velocity -= normal * (velocity_along_normal * num!(2));
                    velocity *= num!(BOUNCE_DAMPING);
                    let overlap = collision_distance - distance;
                    position += normal * overlap * num!(0.5);
                }
            }
        }

        Ok((position, velocity, &self.touched))
    }

    pub fn move_and_collide<
        const MOVING_RADIUS: i32,
        const STATIC_RADIUS: i32,
        const GRAVITY: i32,
        const LEFT_WALL: i32,
        const UP_WALL: i32,
        const RIGHT_WALL: i32,
        const DOWN_WALL: i32,
    >(
        &mut self,
        mut position: Coordinates,
        mut velocity: Force,
        positions: &[Coordinates; N],
        collidable: &[bool; N],
        delta: Fixed,
        walls: &[(Coordinates, Coordinates)],
    ) -> Result<(Coordinates, Force, &[usize]), Error> {
        velocity += Force::new(num!(0), Fixed::new(GRAVITY)) * delta;
        position += velocity * delta;

        (position, velocity) = Self::move_and_collide_with_walls::<
            LEFT_WALL,
            UP_WALL,
            RIGHT_WALL,
            DOWN_WALL,
        >(position, velocity, num!(1));

        for &(wall_start, wall_end) in walls {
            (position, velocity) = Self::move_and_collide_with_given_wall(
                position,
                velocity,
                Fixed::new(MOVING_RADIUS),
                wall_start,
                wall_end,
            );
        }

        let (position, velocity, touched) = self
            .move_and_collide_with_statics::<MOVING_RADIUS, STATIC_RADIUS>(
                position, velocity, positions, collidable,
            )?;

        Ok((position, velocity, touched))
    }

    fn calculate_peg_forces<const REPULSION_STRENGTH: i32>(
        &mut self,
        index: usize,
        this_position: Coordinates,
        this_force_radius: Fixed,
        positions: &[Coordinates; N],
        velocities: &mut [Force; N],
        delta: Fixed,
    ) -> Result<(), Error> {
        let neighbors = self.neighbors.get_neighbors(this_position, 5)?;

        for &i in neighbors {
            if i as usize == index {
                continue;
            }

            let distance_vector = this_position - positions[i as usize];
            let distance_squared = distance_vector.magnitude_squared();

            if distance_squared < num!(0.1) {
                continue;
            }

            if distance_squared < this_force_radius {
                let normal = distance_vector / distance_squared;
                let force_magnitude =
                    Fixed::new(REPULSION_STRENGTH * REPULSION_STRENGTH)
                        / distance_squared;
                let repulsion_force = normal * force_magnitude;
                let velocity = velocities[index] + repulsion_force * delta;
                velocities[index] = vec2(
                    velocity.x.clamp(num!(-50), num!(50)),
                    velocity.y.clamp(num!(-50), num!(50)),
                );
            }
        }

        Ok(())
    }

    fn apply_movement_and_wall_collision<
        const LEFT_WALL: i32,
        const UP_WALL: i32,
        const RIGHT_WALL: i32,
        const DOWN_WALL: i32,
    >(
        index: usize,
        positions: &mut [Coordinates; N],
        velocity: &mut [Force; N],
    ) {
        velocity[index] *= num!(0.98);

        (positions[index], velocity[index]) =
            Self::move_and_collide_with_walls::<
                LEFT_WALL,
                UP_WALL,
                RIGHT_WALL,
                DOWN_WALL,
            >(positions[index], velocity[index], num!(4));
    }

    pub fn move_from_fields<
        const REPULSION_STRENGTH: i32,
        const ATTRACTION_STRENGTH: i32,
        const LEFT_WALL: i32,
        const UP_WALL: i32,
        const RIGHT_WALL: i32,
        const DOWN_WALL: i32,
        const PARTIAL: usize,
    >(
        &mut self,
        positions: &mut [Coordinates; N],
        velocities: &mut [Force; N],
        collidable: &[bool; N],
        force_radius_squared: &[Fixed; N],
        delta: Fixed,
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
                self.calculate_peg_forces::<REPULSION_STRENGTH>(
                    i,
                    position,
                    force,
                    positions,
                    velocities,
                    delta * num!(PARTIAL),
                )?;
            }

            Self::apply_movement_and_wall_collision::<
                LEFT_WALL,
                UP_WALL,
                RIGHT_WALL,
                DOWN_WALL,
            >(i, positions, velocities);

            let new_pos =
                grid::clamp_position_to_grid(position + velocity * delta);

            positions[i] = new_pos;

            if new_pos != position {
                self.neighbors.update(i, position, new_pos)?;
            }
        }

        Ok(())
    }
}
