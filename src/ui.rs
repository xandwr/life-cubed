use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};

use crate::flycam::CameraInfo;

/// A marker component for updating info text.
#[derive(Component)]
pub struct TextChanges;

/// Initializes the info text system.
pub fn ui_text_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("../assets/fonts/roboto.ttf");
    let root_uinode = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::SpaceBetween,

                ..default()
            },
            ..default()
        })
        .id();

    let left_column = commands.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            flex_grow: 1.,
            margin: UiRect::axes(Val::Px(10.0), Val::Px(10.0)),
            ..default()
        },
        ..default()
    }).with_children(|builder| {
        builder.spawn(
            TextBundle::from_section(
                "DEBUG INFO:",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
            )
        );
        builder.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "FPS:\n",
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    "Frame Time:\n",
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    "Camera Position:\n",
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    "Camera Velocity:\n",
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ),
            ]),
            TextChanges,
        ));
    }).id();

    commands
        .entity(root_uinode)
        .push_children(&[left_column]);
}

/// Manages the updating of info text.
pub fn update_ui_text_system(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    camera_info: Res<CameraInfo>,
    mut query: Query<&mut Text, With<TextChanges>>,
) {
    for mut text in &mut query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) =
            diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                frame_time = frame_time_smoothed;
            }
        }

        text.sections[0].value = format!("FPS: {fps:.1} fps\n");
        text.sections[1].value = format!("Frame Time: {frame_time:.3} ms/frame\n");
        text.sections[2].value = format!("Camera Position: {:?}\n", camera_info.position);
        text.sections[3].value = format!("Camera Velocity: {:?}\n", camera_info.velocity);
    }
}