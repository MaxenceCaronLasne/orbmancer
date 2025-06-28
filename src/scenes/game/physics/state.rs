use crate::scenes::game::peg::{PegIndex, Pegs, MAX_PEGS};

use super::constants::PhysicsConfig;
use super::grid::{Grid, GridCoordinate};

const MAX_NEIGHBORS: usize = 16;

pub struct PhysicsState {
    grid: Grid,
    pub config: PhysicsConfig,
    neighbor_buffer: [PegIndex; MAX_NEIGHBORS],
    peg_grid_coords: [Option<GridCoordinate>; MAX_PEGS],
}

impl PhysicsState {
    #[must_use]
    pub fn new(pegs: &Pegs) -> Self {
        let mut peg_grid_coords = [None; MAX_PEGS];
        
        // Initialize grid coordinates for existing pegs
        for i in 0..pegs.count {
            if pegs.is_present(i) {
                if let Ok(grid_coord) = Grid::coord_to_grid(pegs.position(i)) {
                    peg_grid_coords[i as usize] = Some(grid_coord);
                }
            }
        }
        
        Self {
            grid: Grid::new(pegs),
            config: PhysicsConfig::default(),
            neighbor_buffer: [0; MAX_NEIGHBORS],
            peg_grid_coords,
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

    pub fn update_grid(&mut self, pegs: &Pegs) {
        self.grid.update(pegs);
    }

    pub fn update_grid_selective(&mut self, pegs: &Pegs) {
        for i in 0..pegs.count {
            if !pegs.is_present(i) {
                // If peg is no longer present, remove it from grid if it was there
                if let Some(old_coord) = self.peg_grid_coords[i as usize] {
                    self.grid.remove_peg(i, old_coord);
                    self.peg_grid_coords[i as usize] = None;
                }
                continue;
            }

            // Calculate current grid coordinate for this peg
            let current_position = pegs.position(i);
            if let Ok(new_coord) = Grid::coord_to_grid(current_position) {
                match self.peg_grid_coords[i as usize] {
                    Some(old_coord) => {
                        // Peg was previously in grid, check if it moved to different cell
                        if old_coord != new_coord {
                            self.grid.move_peg(i, old_coord, new_coord);
                            self.peg_grid_coords[i as usize] = Some(new_coord);
                        }
                        // If same cell, do nothing
                    }
                    None => {
                        // Peg wasn't in grid before, add it
                        self.grid.add_peg(i, new_coord);
                        self.peg_grid_coords[i as usize] = Some(new_coord);
                    }
                }
            } else {
                // Invalid coordinate, remove from grid if it was there
                if let Some(old_coord) = self.peg_grid_coords[i as usize] {
                    self.grid.remove_peg(i, old_coord);
                    self.peg_grid_coords[i as usize] = None;
                }
            }
        }
    }
}
