use bevy::{input::system::exit_on_esc_system, prelude::*};

pub mod core;
pub mod world_map;

use crate::core::GamePlugins;

fn main()
{
    App::build()
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system.system())
        .add_plugins(GamePlugins)
        .add_startup_system(enter_world_map.system())
        .run();
}

fn enter_world_map(mut state: ResMut<State<core::GameState>>)
{
    state.set(core::GameState::WorldMap).unwrap();
}