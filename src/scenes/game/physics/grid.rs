use crate::types::Fixed;
use crate::scenes::game::peg::Pegs;
use agb::fixnum::num;
use alloc::vec::Vec;

const GRID_BITS: usize = 3; // 8x8 grid
const GRID_SIZE: usize = 1 << GRID_BITS;
pub const GRID_CELLS: usize = GRID_SIZE * GRID_SIZE;
const CELL_SHIFT: usize = 5; // log2(32)
const MAX_PEGS_PER_CELL: usize = 8;
const MAX_INTERACTION_DISTANCE_SQUARED: f32 = 60.0 * 60.0;
const MAX_PEGS: usize = 50;

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

pub trait NeighborStrategy {
    fn get_neighbors(&self, peg_id: usize, pegs: &Pegs) -> Vec<usize>;
}

pub struct GridNeighborStrategy {
    grid: SpatialGrid,
}

impl GridNeighborStrategy {
    pub fn new() -> Self {
        Self {
            grid: SpatialGrid::new(),
        }
    }

    pub fn populate_grid(&mut self, pegs: &Pegs) {
        self.grid.clear();
        for i in 0..pegs.count {
            if pegs.present[i] {
                self.grid.insert(i as u8, pegs.grid_cells[i]);
            }
        }
    }
}

impl NeighborStrategy for GridNeighborStrategy {
    fn get_neighbors(&self, peg_id: usize, pegs: &Pegs) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let cell = pegs.grid_cells[peg_id];
        
        let count = self.grid.cell_counts[cell as usize] as usize;
        for i in 0..count {
            let neighbor_id = self.grid.cells[cell as usize][i] as usize;
            if neighbor_id != peg_id {
                neighbors.push(neighbor_id);
            }
        }

        let neighbor_cells = self.grid.get_neighbor_cells(cell);
        for &neighbor_cell in neighbor_cells.iter() {
            if neighbor_cell == 0xFF {
                break;
            }

            let neighbor_count = self.grid.cell_counts[neighbor_cell as usize] as usize;
            for k in 0..neighbor_count {
                let neighbor_id = self.grid.cells[neighbor_cell as usize][k] as usize;
                neighbors.push(neighbor_id);
            }
        }

        neighbors
    }
}

pub struct NaiveNeighborStrategy;

impl NaiveNeighborStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl NeighborStrategy for NaiveNeighborStrategy {
    fn get_neighbors(&self, peg_id: usize, pegs: &Pegs) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let peg_pos = pegs.positions[peg_id];

        for i in 0..pegs.count {
            if i == peg_id || !pegs.present[i] {
                continue;
            }

            let distance_vector = pegs.positions[i] - peg_pos;
            let distance_squared = distance_vector.magnitude_squared();

            if distance_squared <= num!(MAX_INTERACTION_DISTANCE_SQUARED) {
                neighbors.push(i);
            }
        }

        neighbors
    }
}
