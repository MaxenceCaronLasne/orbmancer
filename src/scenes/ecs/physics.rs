use crate::DELTA;
use crate::position::Position;
use crate::{Fixed, Force};
use agb::fixnum::{Rect, Vector2D, num};
use agb::hash_map::HashSet;
use bevy::prelude::*;
use heapless::Vec as HeaplessVec;

const BOUNCE_DAMPING: f32 = 0.9;
const WALL_BOUNCE_DAMPING: f32 = 0.9;
const MIN_DISTANCE_THRESHOLD: f32 = 0.001;
const OVERLAP_ADJUSTMENT: f32 = 0.5;
const ESCAPE_FORCE: i32 = 15;
const FIELD_RADIUS: i32 = 20;
const FIELD_REPULSION_STRENGTH: i32 = 3000;

// Grid constants
const MAX_ENTITIES_PER_CELL: usize = 16;
const GRID_CELL_SIZE: i32 = 8; // Should be larger than largest radius

//type KineticObjects<'a> = (&'a mut Position, &'a mut Velocity);
type KineticFilter = (With<Kinetic>, Without<Static>);
//type StaticObjects<'a> = &'a Position;
type StaticFilter = (With<Static>, Without<Kinetic>);

#[derive(Component, Clone, Debug)]
#[component(storage = "SparseSet")]
pub struct Kinetic;

#[derive(Component)]
pub struct Static;

#[derive(Component)]
pub struct Field;

#[derive(Component, Clone, Debug)]
pub struct Velocity(Force);

impl Velocity {
    pub fn zero() -> Velocity {
        Velocity(Force::new(num!(0), num!(0)))
    }
}

#[derive(Resource)]
struct KinematicWalls(Rect<Fixed>);

#[derive(Resource)]
struct StaticWalls(Rect<Fixed>);

#[derive(Resource)]
struct KinematicRadius(Fixed);

#[derive(Resource)]
struct StaticRadius(Fixed);

#[derive(Resource)]
struct FieldRadius(Fixed);

#[derive(Resource)]
struct StaticSpatialGrid {
    cells: Vec<Vec<HeaplessVec<Entity, MAX_ENTITIES_PER_CELL>>>,
    width: usize,
    height: usize,
    cell_size: i32,
    bounds: Rect<Fixed>,
}

impl StaticSpatialGrid {
    fn new(bounds: Rect<Fixed>, cell_size: i32) -> Self {
        // Simple integer division for grid size
        let width = (bounds.size.x.floor() / cell_size + 1).max(1) as usize;
        let height = (bounds.size.y.floor() / cell_size + 1).max(1) as usize;

        let mut cells = Vec::with_capacity(width);
        for _ in 0..width {
            let mut column = Vec::with_capacity(height);
            for _ in 0..height {
                column.push(HeaplessVec::new());
            }
            cells.push(column);
        }

        Self {
            cells,
            width,
            height,
            cell_size,
            bounds,
        }
    }

    fn position_to_cell(
        &self,
        position: Vector2D<Fixed>,
    ) -> Option<(usize, usize)> {
        let relative_pos = position - self.bounds.position;

        if relative_pos.x < num!(0)
            || relative_pos.y < num!(0)
            || relative_pos.x >= self.bounds.size.x
            || relative_pos.y >= self.bounds.size.y
        {
            return None;
        }

        let cell_x = (relative_pos.x.floor() / self.cell_size).max(0) as usize;
        let cell_y = (relative_pos.y.floor() / self.cell_size).max(0) as usize;

        if cell_x < self.width && cell_y < self.height {
            Some((cell_x, cell_y))
        } else {
            None
        }
    }

    fn insert(&mut self, entity: Entity, position: Vector2D<Fixed>) {
        if let Some((x, y)) = self.position_to_cell(position) {
            let _ = self.cells[x][y].push(entity);
        }
    }

    fn remove(&mut self, entity: Entity, position: Vector2D<Fixed>) {
        if let Some((x, y)) = self.position_to_cell(position) {
            if let Some(pos) =
                self.cells[x][y].iter().position(|&e| e == entity)
            {
                self.cells[x][y].swap_remove(pos);
            }
        }
    }

    fn get_neighbors(
        &self,
        position: Vector2D<Fixed>,
        radius: Fixed,
    ) -> Vec<Entity> {
        let mut neighbors = Vec::new();

        if let Some((center_x, center_y)) = self.position_to_cell(position) {
            let cell_radius = (radius.floor() / self.cell_size + 1).max(1);

            for dx in -(cell_radius)..=cell_radius {
                for dy in -(cell_radius)..=cell_radius {
                    let x = center_x as i32 + dx;
                    let y = center_y as i32 + dy;

                    if x >= 0
                        && x < self.width as i32
                        && y >= 0
                        && y < self.height as i32
                    {
                        for &entity in &self.cells[x as usize][y as usize] {
                            neighbors.push(entity);
                        }
                    }
                }
            }
        }

        neighbors
    }

    fn clear(&mut self) {
        for column in &mut self.cells {
            for cell in column {
                cell.clear();
            }
        }
    }
}

#[derive(Resource, Clone, Debug)]
struct Gravity(Force);

fn update_spatial_grid(
    mut grid: ResMut<StaticSpatialGrid>,
    added_statics: Query<
        (Entity, &Position),
        (Added<Static>, Without<Kinetic>),
    >,
    changed_statics: Query<
        (Entity, &Position),
        (Changed<Position>, With<Static>, Without<Kinetic>),
    >,
    removed_statics: RemovedComponents<Static>,
    all_statics: Query<(Entity, &Position), (With<Static>, Without<Kinetic>)>,
) {
    // Handle newly added static entities
    for (entity, position) in added_statics.iter() {
        grid.insert(entity, position.0);
    }

    // Handle removed static entities - we need to rebuild the grid since we can't track old positions
    if !removed_statics.is_empty() {
        grid.clear();
        for (entity, position) in all_statics.iter() {
            grid.insert(entity, position.0);
        }
    } else {
        // Handle position changes - rebuild grid for simplicity
        // In a more optimized version, we'd track old positions
        if !changed_statics.is_empty() {
            grid.clear();
            for (entity, position) in all_statics.iter() {
                grid.insert(entity, position.0);
            }
        }
    }
}

fn apply_gravity(
    mut query: Query<&mut Velocity, With<Kinetic>>,
    gravity: Res<Gravity>,
) {
    for mut v in &mut query {
        v.0 += gravity.0; // * num!(DELTA);
    }
}

fn apply_velocity(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut p, v) in &mut query {
        p.0 += v.0 * num!(DELTA);
    }
}

fn apply_kinetic_collision(
    mut kinetics: Query<(&mut Position, &mut Velocity), KineticFilter>,
    statics: Query<&Position, StaticFilter>,
    kinematic_radius: Res<KinematicRadius>,
    static_radius: Res<StaticRadius>,
    walls: Res<KinematicWalls>,
    grid: Res<StaticSpatialGrid>,
) {
    let collision_distance = kinematic_radius.0 + static_radius.0;
    let collision_distance_squared = collision_distance * collision_distance;

    for (mut pk, mut vk) in kinetics.iter_mut() {
        // Handle collisions with static objects using grid
        let neighbors = grid.get_neighbors(pk.0, collision_distance);

        for static_entity in neighbors {
            if let Ok(ps) = statics.get(static_entity) {
                let distance_vector = pk.0 - ps.0;
                let distance_squared = distance_vector.magnitude_squared();

                if distance_squared < collision_distance_squared
                    && distance_squared > num!(MIN_DISTANCE_THRESHOLD)
                {
                    let distance = distance_squared.sqrt();
                    let normal = distance_vector / distance;

                    // Reflect velocity: v' = v - 2(v·n)n
                    let velocity_along_normal = vk.0.dot(normal);
                    vk.0 -= normal * (velocity_along_normal * num!(2.0));
                    vk.0 *= num!(BOUNCE_DAMPING);

                    // Separate overlapping objects
                    let overlap = collision_distance - distance;
                    pk.0 += normal * overlap * num!(OVERLAP_ADJUSTMENT);
                }
            }
        }

        // Handle wall collisions
        let radius = kinematic_radius.0;
        let wall_bounds = walls.0;

        // Left wall
        if pk.0.x < wall_bounds.position.x + radius {
            pk.0.x = wall_bounds.position.x + radius;
            vk.0.x = -vk.0.x * num!(WALL_BOUNCE_DAMPING) + num!(ESCAPE_FORCE);
        }
        // Right wall
        else if pk.0.x > wall_bounds.position.x + wall_bounds.size.x - radius
        {
            pk.0.x = wall_bounds.position.x + wall_bounds.size.x - radius;
            vk.0.x = -vk.0.x * num!(WALL_BOUNCE_DAMPING) - num!(ESCAPE_FORCE);
        }

        // Top wall
        if pk.0.y < wall_bounds.position.y + radius {
            pk.0.y = wall_bounds.position.y + radius;
            vk.0.y = -vk.0.y * num!(WALL_BOUNCE_DAMPING) + num!(ESCAPE_FORCE);
        }
        // Bottom wall
        else if pk.0.y > wall_bounds.position.y + wall_bounds.size.y - radius
        {
            pk.0.y = wall_bounds.position.y + wall_bounds.size.y - radius;
            vk.0.y = -vk.0.y * num!(WALL_BOUNCE_DAMPING) - num!(ESCAPE_FORCE);
        }
    }
}

fn apply_field(
    mut field_objects: Query<(Entity, &mut Velocity, &Position), With<Field>>,
    static_objects: Query<
        &Position,
        (With<Static>, With<Field>, Without<Kinetic>),
    >,
    grid: Res<StaticSpatialGrid>,
    field_radius: Res<FieldRadius>,
) {
    let field_radius_squared = field_radius.0 * field_radius.0;

    // Get all field objects in a vector to avoid borrow checker issues
    let mut field_entities: Vec<(Entity, Vector2D<Fixed>, Force)> =
        field_objects
            .iter()
            .map(|(entity, velocity, position)| {
                (entity, position.0, velocity.0)
            })
            .collect();

    // Process field interactions
    for i in 0..field_entities.len() {
        let (_entity1, position1, mut velocity1) = field_entities[i];

        // Get nearby static field objects from grid
        let neighbors = grid.get_neighbors(position1, field_radius.0);

        for neighbor_entity in neighbors {
            if let Ok(neighbor_position) = static_objects.get(neighbor_entity) {
                let distance_vector = position1 - neighbor_position.0;
                let distance_squared = distance_vector.magnitude_squared();

                if distance_squared < field_radius_squared
                    && distance_squared > num!(MIN_DISTANCE_THRESHOLD)
                {
                    let distance = distance_squared.sqrt();
                    let normal = distance_vector / distance;

                    let force_magnitude =
                        num!(FIELD_REPULSION_STRENGTH) / (distance * distance);
                    let repulsion_force = normal * force_magnitude;

                    velocity1 += repulsion_force * num!(DELTA);
                }
            }
        }

        // Check interactions with other field objects (non-static)
        for j in (i + 1)..field_entities.len() {
            let (_, position2, mut velocity2) = field_entities[j];

            let distance_vector = position1 - position2;
            let distance_squared = distance_vector.magnitude_squared();

            if distance_squared < field_radius_squared
                && distance_squared > num!(MIN_DISTANCE_THRESHOLD)
            {
                let distance = distance_squared.sqrt();
                let normal = distance_vector / distance;

                let force_magnitude =
                    num!(FIELD_REPULSION_STRENGTH) / (distance * distance);
                let repulsion_force = normal * force_magnitude;

                velocity1 += repulsion_force * num!(DELTA);
                velocity2 -= repulsion_force * num!(DELTA);

                field_entities[j].2 = velocity2;
            }
        }

        field_entities[i].2 = velocity1;
    }

    // Apply calculated velocities back to entities
    for (entity, _, new_velocity) in field_entities {
        if let Ok((_, mut velocity, _)) = field_objects.get_mut(entity) {
            velocity.0 = new_velocity;
        }
    }
}

fn apply_static_collision(
    mut statics: Query<(Entity, &mut Position), StaticFilter>,
    static_radius: Res<StaticRadius>,
    walls: Res<StaticWalls>,
    grid: Res<StaticSpatialGrid>,
) {
    let collision_distance = static_radius.0 + static_radius.0;
    let collision_distance_squared = collision_distance * collision_distance;

    // Collect all static entities and their positions
    let mut static_entities: Vec<(Entity, Vector2D<Fixed>)> = statics
        .iter()
        .map(|(entity, position)| (entity, position.0))
        .collect();

    // Handle static-to-static collisions using grid
    let mut processed_pairs = HashSet::new();

    for i in 0..static_entities.len() {
        let (entity1, mut position1) = static_entities[i];
        let neighbors = grid.get_neighbors(position1, collision_distance);

        for neighbor_entity in neighbors {
            if entity1 == neighbor_entity {
                continue;
            }

            // Avoid processing the same pair twice
            let pair = if entity1.index() < neighbor_entity.index() {
                (entity1, neighbor_entity)
            } else {
                (neighbor_entity, entity1)
            };

            if processed_pairs.contains(&pair) {
                continue;
            }
            processed_pairs.insert(pair);

            // Find the neighbor in our static entities list
            if let Some(neighbor_index) = static_entities
                .iter()
                .position(|(e, _)| *e == neighbor_entity)
            {
                let (_, mut position2) = static_entities[neighbor_index];

                let distance_vector = position1 - position2;
                let distance_squared = distance_vector.magnitude_squared();

                if distance_squared < collision_distance_squared
                    && distance_squared > num!(MIN_DISTANCE_THRESHOLD)
                {
                    let distance = distance_squared.sqrt();
                    let normal = distance_vector / distance;

                    // Separate overlapping objects equally
                    let overlap = collision_distance - distance;
                    let separation =
                        normal * overlap * num!(OVERLAP_ADJUSTMENT);

                    position1 += separation;
                    position2 -= separation;

                    static_entities[i].1 = position1;
                    static_entities[neighbor_index].1 = position2;
                }
            }
        }
    }

    // Apply calculated positions back to entities
    for (entity, new_position) in static_entities {
        if let Ok((_, mut position)) = statics.get_mut(entity) {
            position.0 = new_position;
        }
    }

    // Handle static-to-wall collisions
    let radius = static_radius.0;
    let wall_bounds = walls.0;

    for (_, mut position) in statics.iter_mut() {
        // Left wall
        if position.0.x < wall_bounds.position.x + radius {
            position.0.x = wall_bounds.position.x + radius;
        }
        // Right wall
        else if position.0.x
            > wall_bounds.position.x + wall_bounds.size.x - radius
        {
            position.0.x = wall_bounds.position.x + wall_bounds.size.x - radius;
        }

        // Top wall
        if position.0.y < wall_bounds.position.y + radius {
            position.0.y = wall_bounds.position.y + radius;
        }
        // Bottom wall
        else if position.0.y
            > wall_bounds.position.y + wall_bounds.size.y - radius
        {
            position.0.y = wall_bounds.position.y + wall_bounds.size.y - radius;
        }
    }
}

pub struct PhysicsPlugin {
    pub static_walls: (i32, i32, i32, i32),
    pub kinematic_walls: (i32, i32, i32, i32),
    pub gravity: Force,
    pub kinematic_radius: i32,
    pub static_radius: i32,
    pub field_radius: i32,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        let static_walls = Rect::new(
            Vector2D::new(
                Fixed::new(self.static_walls.0),
                Fixed::new(self.static_walls.1),
            ),
            Vector2D::new(
                Fixed::new(self.static_walls.2),
                Fixed::new(self.static_walls.3),
            ),
        );

        app.insert_resource(StaticWalls(static_walls));
        app.insert_resource(KinematicWalls(Rect::new(
            Vector2D::new(
                Fixed::new(self.kinematic_walls.0),
                Fixed::new(self.kinematic_walls.1),
            ),
            Vector2D::new(
                Fixed::new(self.kinematic_walls.2),
                Fixed::new(self.kinematic_walls.3),
            ),
        )));
        app.insert_resource(Gravity(self.gravity));
        app.insert_resource(KinematicRadius(Fixed::new(self.kinematic_radius)));
        app.insert_resource(StaticRadius(Fixed::new(self.static_radius)));
        app.insert_resource(FieldRadius(Fixed::new(self.field_radius)));

        // Initialize the spatial grid
        app.insert_resource(StaticSpatialGrid::new(
            static_walls,
            GRID_CELL_SIZE,
        ));

        app.add_systems(
            Update,
            (
                update_spatial_grid,
                apply_gravity,
                apply_field,
                apply_velocity,
                apply_kinetic_collision,
                apply_static_collision,
            )
                .chain(),
        );
    }
}
