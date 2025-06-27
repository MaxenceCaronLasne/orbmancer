use crate::scenes::game::peg::PegIndex;
use crate::scenes::game::{Ball, peg::Pegs};
use crate::types::Fixed;
use agb::fixnum::{num, vec2};

use super::collision::{handle_ball_wall_collisions};
use super::constants::{PhysicsConfig, SCREEN_BOTTOM, ZERO};
use super::grid::{Grid, NeighborStrategy};

const MAX_NEIGHBORS: usize = 32;

pub struct PhysicsState<T: NeighborStrategy> {
    neighbor_strategy: T,
    config: PhysicsConfig,
    neighbor_buffer: [PegIndex; MAX_NEIGHBORS],
}

impl<T: NeighborStrategy> PhysicsState<T> {
    #[must_use]
    pub fn new(neighbor_strategy: T) -> Self {
        Self {
            neighbor_strategy,
            config: PhysicsConfig::default(),
            neighbor_buffer: [0; MAX_NEIGHBORS],
        }
    }


    #[must_use]
    pub fn config(&self) -> &PhysicsConfig {
        &self.config
    }


    #[inline]
    pub fn fill_neighbors(
        &mut self,
        position: crate::types::Coordinate,
    ) -> usize {
        let mut count = 0;
        for neighbor_id in self.neighbor_strategy.get_neighbors(position) {
            if count >= MAX_NEIGHBORS {
                break;
            }
            self.neighbor_buffer[count] = neighbor_id;
            count += 1;
        }
        count
    }

    #[inline]
    pub fn neighbors(&self, count: usize) -> &[PegIndex] {
        &self.neighbor_buffer[..count.min(MAX_NEIGHBORS)]
    }
}

impl PhysicsState<Grid> {
}

#[must_use]
pub fn new(pegs: &Pegs) -> PhysicsState<Grid> {
    PhysicsState::new(Grid::new(pegs))
}


pub fn update_ball<T: NeighborStrategy>(
    ball: &mut Ball,
    pegs: &mut Pegs,
    delta_time: Fixed,
    state: &mut PhysicsState<T>,
) {
    let initial_position = ball.position;
    let initial_velocity = ball.velocity;

    ball.velocity =
        initial_velocity + vec2(num!(0), state.config.gravity_y) * delta_time;
    ball.position = initial_position + ball.velocity * delta_time;

    handle_ball_wall_collisions(ball);

    if ball.position.y > num!(SCREEN_BOTTOM) {
        ball.velocity = vec2(num!(ZERO), num!(ZERO));
        return;
    }

    super::collision::handle_ball_peg_collisions(ball, pegs, state);
}



