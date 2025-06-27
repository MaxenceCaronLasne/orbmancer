use crate::scenes::game::peg::PegIndex;
use crate::scenes::game::{Ball, peg::Pegs};
use crate::types::Fixed;
use agb::fixnum::{num, vec2};

use super::collision::handle_ball_wall_collisions;
use super::constants::{PhysicsConfig, SCREEN_BOTTOM, ZERO};
use super::grid::Grid;

const MAX_NEIGHBORS: usize = 16;

pub struct PhysicsState {
    grid: Grid,
    config: PhysicsConfig,
    neighbor_buffer: [PegIndex; MAX_NEIGHBORS],
}

impl PhysicsState {
    #[must_use]
    pub fn new(pegs: &Pegs) -> Self {
        Self {
            grid: Grid::new(pegs),
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
        self.grid
            .fill_neighbors(position, &mut self.neighbor_buffer)
    }

    #[inline]
    pub fn neighbors(&self, count: usize) -> &[PegIndex] {
        &self.neighbor_buffer[..count.min(MAX_NEIGHBORS)]
    }
}

pub fn update_ball(
    ball: &mut Ball,
    pegs: &mut Pegs,
    delta_time: Fixed,
    state: &mut PhysicsState,
) {
    let initial_position = ball.position;
    let initial_velocity = ball.velocity;

    crate::bench::start("GRAVITY");
    ball.velocity =
        initial_velocity + vec2(num!(0), state.config.gravity_y) * delta_time;
    crate::bench::stop("GRAVITY");

    crate::bench::start("POSITION_UPDATE");
    ball.position = initial_position + ball.velocity * delta_time;
    crate::bench::stop("POSITION_UPDATE");

    crate::bench::start("WALL");
    handle_ball_wall_collisions(ball);
    crate::bench::stop("WALL");

    if ball.position.y > num!(SCREEN_BOTTOM) {
        ball.velocity = vec2(num!(ZERO), num!(ZERO));
        return;
    }

    super::collision::handle_ball_peg_collisions(ball, pegs, state);
}
