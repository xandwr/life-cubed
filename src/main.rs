#![crate_name = "life_cubed"]

mod cell;
mod world;
mod graphics;

use bevy::prelude::*;
use graphics::setup_graphics;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_graphics)
        .run();
}