use crate::scenes::game::peg::Pegs;
use crate::types::Coordinate;
use crate::{error::Error, scenes::game::peg::PegIndex};
use agb::fixnum::Vector2D;
use agb::hash_map::HashMap;
use alloc::{vec, vec::Vec};
use core::num::TryFromIntError;

const SHIFT_VALUE: u32 = 3;

pub trait NeighborStrategy {
    fn get_neighbors(
        &self,
        coordinate: Coordinate,
    ) -> impl Iterator<Item = PegIndex>;
}

type GridCoordinate = Vector2D<u8>;

pub struct Grid {
    hash_map: HashMap<GridCoordinate, Vec<PegIndex>>,
}

impl Grid {
    pub fn new(pegs: &Pegs) -> Self {
        let mut res = Grid {
            hash_map: HashMap::new(),
        };

        for i in 0..pegs.count {
            if pegs.is_present(i) {
                let _ = res.push(i, pegs.position(i));
            }
        }

        res
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
        index: PegIndex,
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
    ) -> impl Iterator<Item = PegIndex> {
        let gc = match Self::coord_to_grid(coordinate) {
            Ok(coords) => coords,
            Err(_) => return NeighborIter::empty(),
        };

        NeighborIter::new(self, gc)
    }
}

pub struct NeighborIter<'a> {
    grid: &'a Grid,
    center: GridCoordinate,
    offset_x: i8,
    offset_y: i8,
    current_cell_iter: Option<core::slice::Iter<'a, PegIndex>>,
}

impl<'a> NeighborIter<'a> {
    #[inline]
    fn new(grid: &'a Grid, center: GridCoordinate) -> Self {
        Self {
            grid,
            center,
            offset_x: -1,
            offset_y: -1,
            current_cell_iter: None,
        }
    }

    #[inline]
    fn empty() -> Self {
        static EMPTY_GRID: Grid = Grid {
            hash_map: HashMap::new(),
        };
        Self {
            grid: &EMPTY_GRID,
            center: Vector2D::new(0, 0),
            offset_x: 2,
            offset_y: 2,
            current_cell_iter: None,
        }
    }

    #[inline]
    fn advance_to_next_cell(&mut self) -> bool {
        loop {
            self.offset_x += 1;
            if self.offset_x > 1 {
                self.offset_x = -1;
                self.offset_y += 1;
                if self.offset_y > 1 {
                    return false;
                }
            }

            let target_x = match self.offset_x {
                -1 => self.center.x.checked_sub(1),
                0 => Some(self.center.x),
                _ => self.center.x.checked_add(1),
            };

            let target_y = match self.offset_y {
                -1 => self.center.y.checked_sub(1),
                0 => Some(self.center.y),
                _ => self.center.y.checked_add(1),
            };

            if let (Some(x), Some(y)) = (target_x, target_y) {
                let coord = Vector2D::new(x, y);
                if let Some(pegs_in_cell) = self.grid.hash_map.get(&coord) {
                    self.current_cell_iter = Some(pegs_in_cell.iter());
                    return true;
                }
            }
        }
    }
}

impl<'a> Iterator for NeighborIter<'a> {
    type Item = PegIndex;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Try to get next item from current cell
            if let Some(ref mut iter) = self.current_cell_iter {
                if let Some(&peg_id) = iter.next() {
                    return Some(peg_id);
                }
            }

            // Current cell exhausted, move to next cell
            self.current_cell_iter = None;
            if !self.advance_to_next_cell() {
                return None;
            }
        }
    }
}
