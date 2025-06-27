//! Spatial grid optimization for efficient neighbor finding
//!
//! This module implements a spatial hash grid that dramatically improves
//! performance for collision detection and force calculations by reducing
//! the search space from O(n) to O(k) where k is the number of nearby objects.
//!
//! # Algorithm
//!
//! 1. **Grid Mapping**: World coordinates are mapped to grid cells using bit shifting
//! 2. **Cell Storage**: Each cell contains a list of peg IDs in that location  
//! 3. **Neighbor Query**: For any coordinate, check the 9 surrounding cells (3x3 grid)
//! 4. **Efficient Updates**: Grid can be updated incrementally as pegs move
//!
//! # Performance
//!
//! - **Grid cell size**: 2³ = 8 units (optimized for typical peg spacing)
//! - **Memory usage**: O(n) where n = number of pegs
//! - **Query time**: O(k) where k = pegs in nearby cells (~9 cells typically)
//! - **Update time**: O(n) to rebuild entire grid
//!
//! # Example
//!
//! ```rust
//! use crate::scenes::game::physics::grid::{Grid, NeighborStrategy};
//! 
//! let grid = Grid::new(&pegs);
//! 
//! // Find all pegs near a ball's position
//! for peg_id in grid.get_neighbors(ball.position) {
//!     // Only check collision with nearby pegs
//!     check_collision(ball, peg_id);
//! }
//! ```

use crate::scenes::game::peg::Pegs;
use crate::types::Coordinate;
use crate::error::Error;
use agb::fixnum::Vector2D;
use agb::hash_map::HashMap;
use alloc::{vec, vec::Vec};
use core::num::TryFromIntError;

/// Grid cell size as a power of 2 for efficient bit-shift operations
/// 
/// SHIFT_VALUE = 3 means cell size = 2³ = 8 units
/// This size is optimized for typical peg spacing and interaction ranges.
const SHIFT_VALUE: u32 = 3;

pub trait NeighborStrategy {
    fn get_neighbors(
        &self,
        coordinate: Coordinate,
    ) -> impl Iterator<Item = usize>;
}

type GridCoordinate = Vector2D<u8>;

pub struct Grid {
    hash_map: HashMap<GridCoordinate, Vec<usize>>,
}

impl Grid {
    pub fn new(pegs: &Pegs) -> Self {
        let mut res = Grid {
            hash_map: HashMap::new(),
        };

        for i in 0..pegs.count {
            if pegs.present[i] {
                let _ = res.push(i, pegs.positions[i]);
            }
        }

        res
    }

    pub fn update(&mut self, pegs: &Pegs) {
        self.hash_map.clear();
        
        for i in 0..pegs.count {
            if pegs.present[i] {
                let _ = self.push(i, pegs.positions[i]);
            }
        }
    }

    fn coord_to_grid(
        coordinate: Coordinate,
    ) -> Result<GridCoordinate, TryFromIntError> {
        let rounded = coordinate.round();

        let unsigned =
            Vector2D::new(rounded.x.unsigned_abs(), rounded.y.unsigned_abs());

        Ok(GridCoordinate::new(
            (unsigned.x >> SHIFT_VALUE).try_into()?,
            (unsigned.y >> SHIFT_VALUE).try_into()?,
        ))
    }

    fn push(
        self: &mut Grid,
        index: usize,
        coordinate: Coordinate,
    ) -> Result<(), Error> {
        let grid_coords = match Grid::coord_to_grid(coordinate) {
            Ok(coords) => coords,
            Err(_) => return Err(Error::GridError),
        };

        if let Some(v) = self.hash_map.get_mut(&grid_coords) {
            v.push(index);
        } else {
            let _ = self.hash_map.insert(grid_coords, vec![index]);
        }

        Ok(())
    }
}

impl NeighborStrategy for Grid {
    fn get_neighbors(
        &self,
        coordinate: Coordinate,
    ) -> impl Iterator<Item = usize> {
        let gc = match Self::coord_to_grid(coordinate) {
            Ok(coords) => coords,
            Err(_) => return Vec::new().into_iter(),
        };

        [
            (gc.x.checked_sub(1), gc.y.checked_sub(1)),
            (gc.x.checked_sub(1), Some(gc.y)),
            (gc.x.checked_sub(1), gc.y.checked_add(1)),
            (Some(gc.x), gc.y.checked_sub(1)),
            (Some(gc.x), Some(gc.y)),
            (Some(gc.x), gc.y.checked_add(1)),
            (gc.x.checked_add(1), gc.y.checked_sub(1)),
            (gc.x.checked_add(1), Some(gc.y)),
            (gc.x.checked_add(1), gc.y.checked_add(1)),
        ]
        .into_iter()
        .filter_map(|(x, y)| x.zip(y).map(|(x, y)| Vector2D::new(x, y)))
        .filter_map(|coord| self.hash_map.get(&coord))
        .flat_map(|pegs_in_cell| pegs_in_cell.iter().copied())
        .collect::<Vec<_>>()
        .into_iter()
    }
}
