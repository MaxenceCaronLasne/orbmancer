use crate::scenes::game::{ball, peg};
use crate::scenes::game::{ball::Ball, peg::Peg};
use crate::types::{Coordinate, Fixed, Force};
use agb::fixnum::{num, vec2};

const GRAVITY_Y: f32 = 200.0;
const LEFT_WALL: f32 = 0.0;
const RIGHT_WALL: f32 = 160.0;
const SCREEN_BOTTOM: f32 = 180.0;
const WALL_BOUNCE_DAMPING: f32 = 0.9;
const PEG_BOUNCE_DAMPING: f32 = 0.9;
const ZERO: f32 = 0.0;

const INTERACTION_FORCE_STRENGTH: f32 = 50.0;
const MAX_INTERACTION_DISTANCE_SQUARED: f32 = 60.0 * 60.0;
const PEG_MOVEMENT_LEFT_BOUND: f32 = 10.0;
const PEG_MOVEMENT_RIGHT_BOUND: f32 = 150.0;
const PEG_MOVEMENT_TOP_BOUND: f32 = 20.0;
const PEG_MOVEMENT_BOTTOM_BOUND: f32 = 140.0;

const MAX_PEGS: usize = 50;
const GRID_BITS: usize = 3; // 8x8 grid
const GRID_SIZE: usize = 1 << GRID_BITS;
const GRID_CELLS: usize = GRID_SIZE * GRID_SIZE;
const CELL_SHIFT: usize = 5; // log2(32)
const MAX_PEGS_PER_CELL: usize = 8;

// Lookup tables for expensive operations (commented out for now)
// TODO: Implement fast sqrt/inv_sqrt using lookup tables for further optimization
/*
#[unsafe(link_section = ".iwram")]
static SQRT_LUT: [u16; 256] = generate_sqrt_lut();
#[unsafe(link_section = ".iwram")]
static INV_SQRT_LUT: [u16; 256] = generate_inv_sqrt_lut();

const fn generate_sqrt_lut() -> [u16; 256] {
    let mut lut = [0u16; 256];
    let mut i = 0;
    while i < 256 {
        // Approximate sqrt using bit manipulation
        let mut x = i as u32;
        let mut result = 0u32;
        let mut bit = 1u32 << 14;

        while bit > x {
            bit >>= 2;
        }

        while bit != 0 {
            if x >= result + bit {
                x -= result + bit;
                result = (result >> 1) + bit;
            } else {
                result >>= 1;
            }
            bit >>= 2;
        }

        lut[i] = result as u16;
        i += 1;
    }
    lut
}

const fn generate_inv_sqrt_lut() -> [u16; 256] {
    let mut lut = [0u16; 256];
    let mut i = 1;
    while i < 256 {
        // Approximate 1/sqrt(i) * 256 for fixed point
        lut[i] = (256 * 16) / generate_sqrt_lut()[i];
        i += 1;
    }
    lut[0] = 0xFFFF; // Handle division by zero
    lut
}
*/

struct SpatialGrid {
    cells: [[u8; MAX_PEGS_PER_CELL]; GRID_CELLS],
    cell_counts: [u8; GRID_CELLS],
}

impl SpatialGrid {
    fn new() -> Self {
        Self {
            cells: [[0xFF; MAX_PEGS_PER_CELL]; GRID_CELLS],
            cell_counts: [0; GRID_CELLS],
        }
    }

    #[inline(always)]
    fn clear(&mut self) {
        for i in 0..GRID_CELLS {
            self.cell_counts[i] = 0;
        }
    }

    #[inline(always)]
    fn hash_position(x: Fixed, y: Fixed) -> u8 {
        let grid_x = (x.to_raw() >> CELL_SHIFT) & ((GRID_SIZE - 1) as i32);
        let grid_y = (y.to_raw() >> CELL_SHIFT) & ((GRID_SIZE - 1) as i32);
        ((grid_y << GRID_BITS) | grid_x) as u8
    }

    #[inline(always)]
    fn insert(&mut self, peg_id: u8, cell: u8) -> bool {
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
    fn get_neighbor_cells(&self, cell: u8) -> [u8; 4] {
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

struct PegPhysicsData {
    positions: [Coordinate; MAX_PEGS],
    field_sizes: [Fixed; MAX_PEGS],
    grid_cells: [u8; MAX_PEGS],
    present: [bool; MAX_PEGS],
    count: usize,
}

impl PegPhysicsData {
    fn new() -> Self {
        Self {
            positions: [vec2(num!(0.0), num!(0.0)); MAX_PEGS],
            field_sizes: [num!(0.0); MAX_PEGS],
            grid_cells: [0; MAX_PEGS],
            present: [false; MAX_PEGS],
            count: 0,
        }
    }

    fn sync_from_pegs(&mut self, pegs: &[Peg]) {
        self.count = pegs.len().min(MAX_PEGS);

        for i in 0..self.count {
            let peg_active = !pegs[i].is_touched();
            self.present[i] = peg_active;

            if peg_active {
                self.positions[i] = pegs[i].position;
                self.field_sizes[i] = pegs[i].force_radius;
                self.grid_cells[i] = SpatialGrid::hash_position(
                    pegs[i].position.x,
                    pegs[i].position.y,
                );
            }
        }

        for i in self.count..MAX_PEGS {
            self.present[i] = false;
        }
    }

    fn sync_to_pegs(&self, pegs: &mut [Peg]) {
        for i in 0..self.count.min(pegs.len()) {
            if self.present[i] && !pegs[i].is_touched() {
                pegs[i].position = self.positions[i];
            }
        }
    }
}

fn handle_ball_wall_collisions(ball: &mut Ball) {
    let ball_radius = num!(ball::RADIUS);

    if ball.position.x < num!(LEFT_WALL) + ball_radius {
        ball.position.x = num!(LEFT_WALL) + ball_radius;
        ball.velocity.x = -ball.velocity.x * num!(WALL_BOUNCE_DAMPING);
    } else if ball.position.x > num!(RIGHT_WALL) - ball_radius {
        ball.position.x = num!(RIGHT_WALL) - ball_radius;
        ball.velocity.x = -ball.velocity.x * num!(WALL_BOUNCE_DAMPING);
    }
}

fn handle_ball_peg_collisions(ball: &mut Ball, pegs: &mut [Peg]) {
    let ball_radius = num!(ball::RADIUS);
    let peg_radius = num!(peg::RADIUS);

    for peg in pegs.iter_mut() {
        if peg.is_touched() {
            continue;
        }

        let distance_vector = ball.position - peg.position;
        let distance = distance_vector.magnitude();
        let collision_distance = ball_radius + peg_radius;

        if distance < collision_distance && distance > num!(ZERO) {
            peg.touch();
            let normal = distance_vector / distance;
            let velocity_along_normal = ball.velocity.dot(normal);

            ball.velocity -= normal * (velocity_along_normal * num!(2.0));
            ball.velocity *= num!(PEG_BOUNCE_DAMPING);

            let overlap = collision_distance - distance;
            ball.position += normal * overlap;
        }
    }
}

fn update_ball_physics(ball: &mut Ball, pegs: &mut [Peg], delta_time: Fixed) {
    let initial_position = ball.position;
    let initial_velocity = ball.velocity;

    ball.velocity =
        initial_velocity + vec2(num!(0), num!(GRAVITY_Y)) * delta_time;
    ball.position = initial_position + ball.velocity * delta_time;

    handle_ball_wall_collisions(ball);

    if ball.position.y > num!(SCREEN_BOTTOM) {
        ball.velocity = Force::new(num!(ZERO), num!(ZERO));
        return;
    }

    handle_ball_peg_collisions(ball, pegs);
}

fn calculate_peg_interaction_force(
    distance_squared: Fixed,
    field_size_squared: Fixed,
    direction_vector: Coordinate,
) -> Force {
    if distance_squared < field_size_squared {
        -direction_vector * num!(INTERACTION_FORCE_STRENGTH) * num!(0.1)
    } else {
        direction_vector
            * num!(INTERACTION_FORCE_STRENGTH)
            * num!(0.01)
            * num!(1.0)
            / distance_squared
    }
}

fn update_peg_physics_optimized(
    physics_data: &mut PegPhysicsData,
    delta_time: Fixed,
) {
    if physics_data.count < 2 {
        return;
    }

    let mut grid = SpatialGrid::new();
    let mut force_buffer = [vec2(num!(0.0), num!(0.0)); MAX_PEGS];

    grid.clear();
    for i in 0..physics_data.count {
        if physics_data.present[i] {
            grid.insert(i as u8, physics_data.grid_cells[i]);
        }
    }

    for cell_idx in 0..GRID_CELLS {
        let count = grid.cell_counts[cell_idx] as usize;
        if count == 0 {
            continue;
        }

        for i in 0..count {
            let id_a = grid.cells[cell_idx][i] as usize;
            if !physics_data.present[id_a] {
                continue;
            }

            for j in (i + 1)..count {
                let id_b = grid.cells[cell_idx][j] as usize;
                if !physics_data.present[id_b] {
                    continue;
                }

                apply_peg_force_pair(
                    physics_data,
                    &mut force_buffer,
                    id_a,
                    id_b,
                );
            }

            let neighbors = grid.get_neighbor_cells(cell_idx as u8);
            for &neighbor_cell in neighbors.iter() {
                if neighbor_cell == 0xFF {
                    break;
                }

                let neighbor_count =
                    grid.cell_counts[neighbor_cell as usize] as usize;
                for k in 0..neighbor_count {
                    let id_b = grid.cells[neighbor_cell as usize][k] as usize;
                    if !physics_data.present[id_b] {
                        continue;
                    }

                    apply_peg_force_pair(
                        physics_data,
                        &mut force_buffer,
                        id_a,
                        id_b,
                    );
                }
            }
        }
    }

    for i in 0..physics_data.count {
        if !physics_data.present[i] {
            continue;
        }

        let velocity_change = force_buffer[i] * delta_time;
        physics_data.positions[i] += velocity_change;

        physics_data.positions[i].x = physics_data.positions[i].x.clamp(
            num!(PEG_MOVEMENT_LEFT_BOUND),
            num!(PEG_MOVEMENT_RIGHT_BOUND),
        );
        physics_data.positions[i].y = physics_data.positions[i].y.clamp(
            num!(PEG_MOVEMENT_TOP_BOUND),
            num!(PEG_MOVEMENT_BOTTOM_BOUND),
        );

        physics_data.grid_cells[i] = SpatialGrid::hash_position(
            physics_data.positions[i].x,
            physics_data.positions[i].y,
        );
    }
}

#[inline(always)]
fn apply_peg_force_pair(
    physics_data: &PegPhysicsData,
    force_buffer: &mut [Force; MAX_PEGS],
    id_a: usize,
    id_b: usize,
) {
    let distance_vector =
        physics_data.positions[id_b] - physics_data.positions[id_a];
    let distance_squared = distance_vector.magnitude_squared();

    if distance_squared > num!(MAX_INTERACTION_DISTANCE_SQUARED) {
        return;
    }

    let field_size_a_squared =
        physics_data.field_sizes[id_a] * physics_data.field_sizes[id_a];
    let field_size_b_squared =
        physics_data.field_sizes[id_b] * physics_data.field_sizes[id_b];

    let force_on_a = calculate_peg_interaction_force(
        distance_squared,
        field_size_a_squared,
        distance_vector,
    );
    let force_on_b = calculate_peg_interaction_force(
        distance_squared,
        field_size_b_squared,
        -distance_vector,
    );

    force_buffer[id_a] += force_on_a;
    force_buffer[id_b] += force_on_b;
}

pub fn update_peg_physics(pegs: &mut [Peg], delta_time: Fixed) {
    let mut physics_data = PegPhysicsData::new();
    physics_data.sync_from_pegs(pegs);
    update_peg_physics_optimized(&mut physics_data, delta_time);
    physics_data.sync_to_pegs(pegs);
}

pub fn update_physics(ball: &mut Ball, pegs: &mut [Peg], delta_time: Fixed) {
    update_ball_physics(ball, pegs, delta_time);
    update_peg_physics(pegs, delta_time);
}

pub fn move_and_collide(ball: &mut Ball, pegs: &mut [Peg], delta_time: Fixed) {
    update_ball_physics(ball, pegs, delta_time);
}

