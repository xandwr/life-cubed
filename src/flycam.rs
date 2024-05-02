use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_atmosphere::plugin::AtmosphereCamera;

use crate::cell_world::CellWorldSettings;

/// Manages the current input state.
#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32
}

/// Manages the camera movement settings.
#[derive(Resource)]
struct CameraMovementSettings {
    pub sensitivity: f32,
    pub fly_speed: f32,
    pub fly_sprint_speed: f32
}

impl Default for CameraMovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00015,
            fly_speed: 30.0,
            fly_sprint_speed: 50.0
        }
    }
}

/// Stores relevant information about the current camera.
#[derive(Resource, Default)]
pub struct CameraInfo {
    pub position: Vec3,
    pub velocity: Vec3
}

/// A marker component used to only rotate the flycam itself.
#[derive(Component)]
pub struct FlyCam;

/// Sets the cursor grab mode to the provided [`CursorGrabMode`].
fn grab_cursor(window: &mut Window, grab_mode: CursorGrabMode) {
    match grab_mode {
        CursorGrabMode::Confined => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

/// Spawns the [`Camera3dBundle`] to be controlled.
fn setup_flycam(
    mut commands: Commands,
    settings: Res<CellWorldSettings>,
    mut input_state: ResMut<InputState>
) {
    let camera_position: Vec3 = settings.origin.translation + Vec3::new(
        settings.origin.translation.x + settings.dimensions.x + 20.0, 
        settings.origin.translation.y + settings.dimensions.y + 20.0, 
        settings.origin.translation.z + settings.dimensions.z + 20.0
    );
    let look_target: Vec3 = settings.origin.translation;
    let look_direction: Vec3 = (look_target - camera_position).normalize();
    input_state.pitch = look_direction.y.asin(); // asin gives the pitch angle only if the input is the vertical component (y)
    input_state.yaw = -look_direction.x.atan2(-look_direction.z); // atan2 gives the full range angle from x to z

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(camera_position).looking_at(look_target, Vec3::Y),
            ..default()
        }, 
        AtmosphereCamera::default(),
        FlyCam
    ));
}

/// Handles keyboard input and movement.
fn flycam_move(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<CameraMovementSettings>,
    mut camera_info: ResMut<CameraInfo>,
    mut query: Query<&mut Transform, With<FlyCam>>
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            let mut velocity: Vec3 = Vec3::ZERO;
            let mut current_fly_speed: f32 = settings.fly_speed;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0.0, local_z.z);
            let right = Vec3::new(local_z.z, 0.0, -local_z.x);

            for key in keys.get_pressed() {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => match key {
                        KeyCode::ShiftLeft => current_fly_speed = settings.fly_sprint_speed,
                        KeyCode::KeyW => velocity += forward,
                        KeyCode::KeyS => velocity -= forward,
                        KeyCode::KeyA => velocity -= right,
                        KeyCode::KeyD => velocity += right,
                        KeyCode::Space => velocity += Vec3::Y,
                        KeyCode::ControlLeft => velocity -= Vec3::Y,
                        _ => ()
                    }
                }
            }
            velocity = velocity.normalize_or_zero();
            transform.translation += velocity * time.delta_seconds() * current_fly_speed;
            camera_info.position = transform.translation;
            camera_info.velocity = velocity * time.delta_seconds() * current_fly_speed;
        }
    } else {
        warn!("Primary window not found for flycam_move!");
    }
}

/// Handles camera rotation when the cursor is grabbing the window.
fn flycam_look(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<CameraMovementSettings>,
    mut query: Query<&mut Transform, With<FlyCam>>,
    mut input_state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>
) {
    if let Ok(window) = primary_window.get_single() {
        let delta_state = input_state.as_mut();
        for mut transform in query.iter_mut() {
            for event in delta_state.reader_motion.read(&motion) {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let window_scale: f32 = window.height().min(window.width());
                        delta_state.pitch -= (settings.sensitivity * event.delta.y * window_scale).to_radians();
                        delta_state.yaw -= (settings.sensitivity * event.delta.x * window_scale).to_radians();
                    }
                }
            }
            delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);
            transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw) * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
        }
    } else {
        warn!("Primary window not found for flycam_look!");
    }
}

/// Handles the cursor window grabbing logic.
/// Currently only allows camera movement while left click is held down.
fn cursor_grab(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if mouse_buttons.pressed(MouseButton::Left) {
            grab_cursor(&mut window, CursorGrabMode::Confined);
        } else {
            grab_cursor(&mut window, CursorGrabMode::None);
        }
    } else {
        warn!("Primary window not found for cursor_grab!");
    }
}

/// Provides an easy method of adding a flycam to a scene via plugins.
#[derive(Resource, Default)]
pub struct FlycamPlugin;

impl Plugin for FlycamPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraInfo::default());
        app.init_resource::<InputState>();
        app.init_resource::<CameraMovementSettings>();
        app.add_systems(Startup, setup_flycam);
        app.add_systems(Update, cursor_grab.after(setup_flycam));
        app.add_systems(Update, flycam_move.after(setup_flycam));
        app.add_systems(Update, flycam_look.after(setup_flycam));
    }
}