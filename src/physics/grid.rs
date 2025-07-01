use agb::fixnum::Vector2D;
use alloc::vec::Vec;

use crate::Coordinates;
use crate::error::Error;

const MAX_NB_NEIGHBOR_PER_CELL: usize = 8;
const NB_WIDTH: usize = 20;
const NB_HEIGHT: usize = 20;
const SHIFT_VALUE: usize = 3;

pub struct Grid2D<Index = u8> {
    grid: [[[Option<Index>; MAX_NB_NEIGHBOR_PER_CELL]; NB_HEIGHT]; NB_WIDTH],
    neighbors: Vec<Index>,
}

impl<Index> Grid2D<Index>
where
    Index: TryFrom<usize> + Default + Copy + PartialEq,
{
    fn spatial_to_grid_coords(
        coordinates: &Coordinates,
    ) -> Result<(usize, usize), Error> {
        let round = coordinates.round();

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
        cell: &mut [Option<Index>],
    ) -> Result<(), Error> {
        for c in cell.iter_mut() {
            if c.is_none() {
                *c = Some(index);
                return Ok(());
            }
        }

        Err(Error::CellFull)
    }

    fn remove_in_cell(
        index: Index,
        cell: &mut [Option<Index>],
    ) -> Result<(), Error> {
        for c in cell.iter_mut() {
            if let Some(i) = c
                && *i == index
            {
                *c = None;
                return Ok(());
            }
        }

        Err(Error::NeighborNotFound)
    }

    pub fn new(
        positions: &[Coordinates],
        collidable: &[bool],
    ) -> Result<Self, Error> {
        let mut res = Self {
            grid: core::array::from_fn(|_| {
                core::array::from_fn(|_| [None; MAX_NB_NEIGHBOR_PER_CELL])
            }),
            neighbors: Vec::new(),
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
        Self::remove_in_cell(index, &mut self.grid[ox][oy])?;

        let (nx, ny) = Self::spatial_to_grid_coords(&new_position)?;
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

                for &i in self.grid[x][y].iter().flatten() {
                    self.neighbors.push(i);
                }
            }
        }

        Ok(&self.neighbors)
    }
}
