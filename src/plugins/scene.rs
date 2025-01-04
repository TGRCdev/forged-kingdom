use bevy::prelude::*;

use crate::systems::scene::*;

#[derive(Default, Debug)]
pub struct ScenePlugin;

impl Plugin for ScenePlugin
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_level);
    }
}