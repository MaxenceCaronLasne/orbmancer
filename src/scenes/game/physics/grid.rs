use crate::scenes::game::peg::Pegs;
use crate::types::Coordinate;
use crate::{error::Error, scenes::game::peg::PegIndex};
use agb::fixnum::Vector2D;
use agb::hash_map::HashMap;
use alloc::{vec, vec::Vec};
use core::num::TryFromIntError;

const SHIFT_VALUE: u32 = 3;

pub type GridCoordinate = Vector2D<u8>;

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

    pub fn coord_to_grid(
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

    pub fn update(&mut self, pegs: &Pegs) {
        self.hash_map.clear();
        for i in 0..pegs.count {
            if pegs.is_present(i) {
                let _ = self.push(i, pegs.position(i));
            }
        }
    }

    pub fn remove_peg(&mut self, peg_id: PegIndex, grid_coord: GridCoordinate) {
        if let Some(pegs_in_cell) = self.hash_map.get_mut(&grid_coord) {
            pegs_in_cell.retain(|&id| id != peg_id);
            if pegs_in_cell.is_empty() {
                self.hash_map.remove(&grid_coord);
            }
        }
    }

    pub fn add_peg(&mut self, peg_id: PegIndex, grid_coord: GridCoordinate) {
        if let Some(pegs_in_cell) = self.hash_map.get_mut(&grid_coord) {
            pegs_in_cell.push(peg_id);
        } else {
            let _ = self.hash_map.insert(grid_coord, vec![peg_id]);
        }
    }

    pub fn move_peg(&mut self, peg_id: PegIndex, old_coord: GridCoordinate, new_coord: GridCoordinate) {
        if old_coord != new_coord {
            self.remove_peg(peg_id, old_coord);
            self.add_peg(peg_id, new_coord);
        }
    }

    pub fn fill_neighbors(
        &self,
        position: Coordinate,
        buffer: &mut [PegIndex; 16],
    ) -> usize {
        let center = match Self::coord_to_grid(position) {
            Ok(coords) => coords,
            Err(_) => return 0,
        };

        let mut count = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                let target_x = center.x.checked_add_signed(dx);
                let target_y = center.y.checked_add_signed(dy);

                if let (Some(x), Some(y)) = (target_x, target_y) {
                    let cell_coord = Vector2D::new(x, y);
                    let maybe_pegs = self.hash_map.get(&cell_coord);
                    if let Some(pegs) = maybe_pegs {
                        for &peg_id in pegs {
                            if count >= buffer.len() {
                                return count;
                            }
                            buffer[count] = peg_id;
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }
}
