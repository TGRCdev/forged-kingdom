use bevy::prelude::*;
use avian3d::prelude::*;

mod player;
use player::*;

mod scene;

pub struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PlayerPlugin)
            .add_plugins(PhysicsPlugins::default())
            .add_plugins(PhysicsDebugPlugin::default())
            .add_systems(Startup, scene::add_test_scene);
    }
}