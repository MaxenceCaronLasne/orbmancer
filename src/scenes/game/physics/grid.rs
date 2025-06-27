use crate::types::Fixed;

const GRID_BITS: usize = 3; // 8x8 grid
const GRID_SIZE: usize = 1 << GRID_BITS;
pub const GRID_CELLS: usize = GRID_SIZE * GRID_SIZE;
const CELL_SHIFT: usize = 5; // log2(32)
const MAX_PEGS_PER_CELL: usize = 8;

pub struct SpatialGrid {
    pub cells: [[u8; MAX_PEGS_PER_CELL]; GRID_CELLS],
    pub cell_counts: [u8; GRID_CELLS],
}

impl SpatialGrid {
    pub fn new() -> Self {
        Self {
            cells: [[0xFF; MAX_PEGS_PER_CELL]; GRID_CELLS],
            cell_counts: [0; GRID_CELLS],
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        for i in 0..GRID_CELLS {
            self.cell_counts[i] = 0;
        }
    }

    #[inline(always)]
    pub fn hash_position(x: Fixed, y: Fixed) -> u8 {
        let grid_x = (x.to_raw() >> CELL_SHIFT) & ((GRID_SIZE - 1) as i32);
        let grid_y = (y.to_raw() >> CELL_SHIFT) & ((GRID_SIZE - 1) as i32);
        ((grid_y << GRID_BITS) | grid_x) as u8
    }

    #[inline(always)]
    pub fn insert(&mut self, peg_id: u8, cell: u8) -> bool {
        let cell_idx = cell as usize;
        let count = self.cell_counts[cell_idx] as usize;
        if count < MAX_PEGS_PER_CELL {
            self.cells[cell_idx][count] = peg_id;
            self.cell_counts[cell_idx] += 1;
            true
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn get_neighbor_cells(&self, cell: u8) -> [u8; 4] {
        let x = cell & ((GRID_SIZE - 1) as u8);
        let y = cell >> GRID_BITS;

        let mut neighbors = [0xFF; 4];
        let mut idx = 0;

        if x < (GRID_SIZE - 1) as u8 {
            neighbors[idx] = cell + 1;
            idx += 1;
        }

        if y < (GRID_SIZE - 1) as u8 {
            neighbors[idx] = cell + GRID_SIZE as u8;
            idx += 1;
        }

        if x < (GRID_SIZE - 1) as u8 && y < (GRID_SIZE - 1) as u8 {
            neighbors[idx] = cell + GRID_SIZE as u8 + 1;
            idx += 1;
        }

        if x > 0 && y < (GRID_SIZE - 1) as u8 {
            neighbors[idx] = cell + GRID_SIZE as u8 - 1;
        }

        neighbors
    }
}
