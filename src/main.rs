#![feature(used_with_arg)]

use crate::plugins::game::GamePlugins;
use bevy::prelude::*;

// Modules

pub mod components;
pub mod plugins;
pub mod resources;
pub mod systems;
pub mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugins)
        .run();
}
