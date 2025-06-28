use crate::scenes::game::peg::PegIndex;
use crate::scenes::game::peg::Pegs;

use super::constants::PhysicsConfig;
use super::grid::Grid;

const MAX_NEIGHBORS: usize = 16;

pub struct PhysicsState {
    grid: Grid,
    pub config: PhysicsConfig,
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
