#![crate_name = "life_cubed"]

mod cell_world;
mod ui;
mod flycam;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_atmosphere::prelude::AtmospherePlugin;
use flycam::FlycamPlugin;
use cell_world::CellWorldPlugin;
use ui::{ui_text_system, update_ui_text_system};

fn main() {
    // Create a new App container
    let mut app = App::new();
    
    // Add required plugins to the app, in the specified order
    app.add_plugins((
        DefaultPlugins, 
        FrameTimeDiagnosticsPlugin,
        CellWorldPlugin, 
        FlycamPlugin, 
        AtmospherePlugin
    ));

    // Add required systems to the app
    app.add_systems(Startup, ui_text_system);
    app.add_systems(Update, update_ui_text_system);

    // Run the app
    app.run();
}