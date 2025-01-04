use crate::plugins::game::GamePlugins;
use bevy::prelude::*;

// Modules

pub mod components;
pub mod plugins;
pub mod resources;
pub mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugins)
        .run();
}
