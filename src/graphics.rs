use bevy::{pbr::{CascadeShadowConfigBuilder, NotShadowCaster}, prelude::*};
use crate::world::World;

/// Initializes the graphics processor with Bevy (and a new world, for now).
pub fn setup_graphics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let mut world = World::new([25, 25, 25]);
    world.initialize();

    // Camera that scales its position depending on the world size (for debug visibility)
    let camera_position = Vec3 {
        x: world.size[0] as f32 * 2.0, 
        y: world.size[1] as f32 * 2.0, 
        z: world.size[2] as f32 * 2.0
    };
    let camera = Camera3dBundle {
        transform: Transform::from_translation(camera_position)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };
    commands.spawn(camera);

    // Sky
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 0.8, 1.0),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(camera_position.distance(Vec3::ZERO) * 2.0)),
            ..default()
        },
        NotShadowCaster,
    ));

    // Generate the shadow map proportional to the scene size
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
    .build();

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(1000.0, 1000.0, 500.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        cascade_shadow_config: cascade_shadow_config,
        ..default()
    });

    // Cell instantiation using 'world'
    let cell_mesh = Cuboid::new(1.0, 1.0, 1.0);
    for x in 0..world.size[0] {
        for y in 0..world.size[1] {
            for z in 0..world.size[2] {
                let cell = &world.grid[x][y][z];
                if cell.is_alive {
                    commands.spawn(PbrBundle {
                        mesh: meshes.add(cell_mesh),
                        material: materials.add(Color::rgb(cell.color[0], cell.color[1], cell.color[2])),
                        transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                        ..Default::default()
                    });
                }
            }
        }
    }
}