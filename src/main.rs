use bevy::prelude::*;

use crate::plugins::game::GamePlugins;

// Modules

pub mod components;
pub mod plugins;
pub mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugins)
        .run();
}
