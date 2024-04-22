use bevy::prelude::*;


// Cube generation settings
const DIMENSIONS: CubeArrayDimensions = CubeArrayDimensions { width: 5, height: 5, depth: 5 };
const BLOCK_SPACING: f32 = 0.15;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}


fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        });

    // Lighting
    commands.spawn(SpotLightBundle {
        spot_light: SpotLight {
            color: Color::WHITE,
            intensity: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(1.0, 2.0, 1.0),
        ..default()
    });

    // Cube Generation
    generate_cubes(commands, meshes, materials, DIMENSIONS, BLOCK_SPACING);
}


struct CubeArrayDimensions {
    width: i32,
    height: i32,
    depth: i32
}


fn generate_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    dimensions: CubeArrayDimensions,
    block_spacing: f32,
) {
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    for x in 0..dimensions.width {
        for y in 0..dimensions.height {
            for z in -dimensions.depth..0 {
                commands.spawn(PbrBundle {
                    mesh: cube.clone(),
                    material: materials.add(Color::WHITE),
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * (1.0 + block_spacing), 
                        y as f32 * (1.0 + block_spacing), 
                        z as f32 * (1.0 + block_spacing)
                    )),
                    ..default()
                });
            }
        }
    }
}