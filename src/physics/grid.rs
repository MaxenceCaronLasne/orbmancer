use core::fmt::Debug;

use agb::fixnum::{Vector2D, num, vec2};
use heapless::Vec as HeaplessVec;

use crate::Coordinates;
use crate::error::Error;

const MAX_NB_NEIGHBOR_PER_CELL: usize = 8;
const NB_WIDTH: usize = 24;
const NB_HEIGHT: usize = 20;
const SHIFT_VALUE: usize = 3;

pub struct Grid2D<Index = u8> {
    grid: [[HeaplessVec<Index, MAX_NB_NEIGHBOR_PER_CELL>; NB_HEIGHT]; NB_WIDTH],
    neighbors: HeaplessVec<Index, 64>,
}

impl<Index> Grid2D<Index>
where
    Index: TryFrom<usize> + Default + Copy + PartialEq + Debug,
{
    fn spatial_to_grid_coords(
        coordinates: &Coordinates,
    ) -> Result<(usize, usize), Error> {
        const GRID_MAX_COORD: i32 =
            ((NB_WIDTH - 1) * (1 << SHIFT_VALUE)) as i32;
        const GRID_MIN_COORD: i32 = 0;

        let clamped_x = coordinates
            .x
            .clamp(num!(GRID_MIN_COORD), num!(GRID_MAX_COORD));
        let clamped_y = coordinates
            .y
            .clamp(num!(GRID_MIN_COORD), num!(GRID_MAX_COORD));

        let round = vec2(clamped_x, clamped_y).round();
        let unsigned =
            Vector2D::new(round.x.unsigned_abs(), round.y.unsigned_abs());

        match (
            (unsigned.x >> SHIFT_VALUE).try_into(),
            (unsigned.y >> SHIFT_VALUE).try_into(),
        ) {
            (Err(_), _) | (_, Err(_)) => Err(Error::Convertion),
            (Ok(x), Ok(y)) => Ok((x, y)),
        }
    }

    fn insert_in_cell(
        index: Index,
        cell: &mut HeaplessVec<Index, MAX_NB_NEIGHBOR_PER_CELL>,
    ) -> Result<(), Error> {
        cell.push(index).map_err(|_| Error::CellFull)
    }

    fn try_remove_in_cell(
        index: Index,
        cell: &mut HeaplessVec<Index, MAX_NB_NEIGHBOR_PER_CELL>,
    ) -> Result<bool, Error> {
        if let Some(pos) = cell.iter().position(|&x| x == index) {
            cell.swap_remove(pos);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn remove_in_cell(
        index: Index,
        cell: &mut HeaplessVec<Index, MAX_NB_NEIGHBOR_PER_CELL>,
    ) -> Result<(), Error> {
        if Self::try_remove_in_cell(index, cell)? {
            Ok(())
        } else {
            agb::println!("ERROR: Object {:?} not found in cell", index);
            agb::println!("Cell contents: {:?}", cell.as_slice());
            agb::println!("Cell has {} objects", cell.len());
            Err(Error::NeighborNotFound)
        }
    }

    pub fn new(
        positions: &[Coordinates],
        collidable: &[bool],
    ) -> Result<Self, Error> {
        let grid: [[HeaplessVec<Index, MAX_NB_NEIGHBOR_PER_CELL>; NB_HEIGHT];
            NB_WIDTH] = Default::default();

        let mut res = Self {
            grid,
            neighbors: HeaplessVec::new(),
        };

        for (i, p) in positions.iter().enumerate() {
            if !collidable[i] {
                continue;
            }

            let (x, y) = Self::spatial_to_grid_coords(p)?;
            let index = Index::try_from(i).map_err(|_| Error::IndexTooBig)?;
            Self::insert_in_cell(index, &mut res.grid[x][y])?;
        }

        Ok(res)
    }

    pub fn insert(
        &mut self,
        index: usize,
        position: Coordinates,
    ) -> Result<(), Error> {
        let index = Index::try_from(index).map_err(|_| Error::IndexTooBig)?;
        let (x, y) = Self::spatial_to_grid_coords(&position)?;
        Self::insert_in_cell(index, &mut self.grid[x][y])?;
        
        Ok(())
    }

    pub fn update(
        &mut self,
        index: usize,
        old_position: Coordinates,
        new_position: Coordinates,
    ) -> Result<(), Error> {
        if old_position == new_position {
            return Ok(());
        }

        let index = Index::try_from(index).map_err(|_| Error::IndexTooBig)?;

        let (ox, oy) = Self::spatial_to_grid_coords(&old_position)?;
        let (nx, ny) = Self::spatial_to_grid_coords(&new_position)?;
        
        let was_removed = Self::try_remove_in_cell(index, &mut self.grid[ox][oy])?;
        if !was_removed {
            // Object wasn't in expected cell, treating as new object (this is normal for newly spawned objects)
        }
        
        Self::insert_in_cell(index, &mut self.grid[nx][ny])?;

        Ok(())
    }

    pub fn get_neighbors(
        &mut self,
        position: Coordinates,
        cell_radius: usize,
    ) -> Result<&[Index], Error> {
        self.neighbors.clear();

        let (cx, cy) = Self::spatial_to_grid_coords(&position)?;

        for x in cx.saturating_sub(cell_radius)..=cx + cell_radius {
            for y in cy.saturating_sub(cell_radius)..=cy + cell_radius {
                if x >= NB_WIDTH || y >= NB_HEIGHT {
                    continue;
                }

                for &i in self.grid[x][y].iter() {
                    self.neighbors
                        .push(i)
                        .map_err(|_| Error::TooManyNeighbors)?;
                }
            }
        }

        Ok(&self.neighbors)
    }
}

pub fn clamp_position_to_grid(position: Coordinates) -> Coordinates {
    const GRID_MAX_COORD: i32 = ((NB_WIDTH - 1) * (1 << SHIFT_VALUE)) as i32;
    const GRID_MIN_COORD: i32 = 0;

    let clamped_x =
        position.x.clamp(num!(GRID_MIN_COORD), num!(GRID_MAX_COORD));
    let clamped_y =
        position.y.clamp(num!(GRID_MIN_COORD), num!(GRID_MAX_COORD));

    vec2(clamped_x, clamped_y)
}
