use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResource;
use rand::{thread_rng, Rng};

/// Represents a cell in the world grid.
#[derive(Component, Clone, Debug)]
pub struct Cell {
    pub position: Vec3,
    pub is_alive: bool,
    pub color: Color
}

/// Manages the grid of cells in the world.
#[derive(Resource)]
pub struct CellGrid {
    pub grid: Vec<Vec<Vec<Cell>>>
}

#[derive(Component)]
struct CellMesh;

/// Manages the global world settings.
#[derive(Resource, ExtractResource, Debug, Clone, Copy)]
pub struct CellWorldSettings {
    pub origin: Transform,
    pub dimensions: Vec3,
}

impl Default for CellWorldSettings {
    fn default() -> Self {
        Self {
            origin: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            dimensions: Vec3::from_array([100.0, 100.0, 100.0])
        }
    }
}

/// A `Plugin` that adds the prerequisites for the world.
#[derive(Debug, Clone, Copy)]
pub struct CellWorldPlugin;

impl Plugin for CellWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CellWorldSettings::default())
            .add_systems(Startup, initialize_cell_grid)
            .add_systems(Startup, spawn_cuboids.after(initialize_cell_grid))
            .add_systems(Update, update_cell_grid);
    }
}

/// Initializes a new CellGrid using the CellWorldSettings default values.
fn initialize_cell_grid(
    mut commands: Commands,
    settings: Res<CellWorldSettings>
) {
    let origin = settings.origin.translation;
    let dimensions = settings.dimensions.as_ivec3();
    let mut grid = vec![vec![vec![Cell {
        position: Vec3::ZERO,
        is_alive: false,
        color: Color::WHITE
    }; dimensions.z as usize]; dimensions.y as usize]; dimensions.x as usize];

    for x in 0..dimensions.x {
        for y in 0..dimensions.y {
            for z in 0..dimensions.z {
                grid[x as usize][y as usize][z as usize] = Cell {
                    position: origin + Vec3::new(x as f32, y as f32, z as f32),
                    is_alive: thread_rng().gen_bool(1.0 / 1000.0),
                    color: Color::GREEN,
                };
            }
        }
    }

    commands.insert_resource(CellGrid { grid });
}

/// Instantiates the Cuboid objects in 3D space corresponding to each living cell in the grid.
fn spawn_cuboids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cell_grid: Res<CellGrid>
) {
    for x in 0..cell_grid.grid.len() {
        for y in 0..cell_grid.grid[x].len() {
            for z in 0..cell_grid.grid[x][y].len() {
                let cell = &cell_grid.grid[x][y][z];
                if cell.is_alive {
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                            material: materials.add(cell.color),
                            transform: Transform::from_translation(cell.position),
                            ..default()
                        },
                        CellMesh
                    ));
                }
            }
        }
    }
}

fn update_cell_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cell_grid: Res<CellGrid>,
    query: Query<Entity, With<CellMesh>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for x in 0..cell_grid.grid.len() {
        for y in 0..cell_grid.grid[x].len() {
            for z in 0..cell_grid.grid[x][y].len() {
                let mut cell = cell_grid.grid[x][y][z].clone();
                cell.is_alive = thread_rng().gen_bool(1.0 / 1000.0);
                if cell.is_alive {
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                            material: materials.add(cell.color),
                            transform: Transform::from_translation(cell.position),
                            ..default()
                        },
                        CellMesh
                    ));
                }
            }
        }
    }
}