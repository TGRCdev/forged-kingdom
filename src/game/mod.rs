use bevy::prelude::*;
use avian2d::prelude::*;

pub mod ship;
use ship::ShipPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ShipPlugin)
            .add_plugins(PhysicsPlugins::default())
            .add_plugins(PhysicsDebugPlugin::default())
            .add_systems(Startup, add_camera);
    }
}

fn add_camera(mut commands: Commands)
{
    commands.spawn(Camera2d);
}