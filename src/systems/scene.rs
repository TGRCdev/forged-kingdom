use bevy::prelude::*;
use blenvy::{BlueprintInfo, GameWorldTag, HideUntilReady, SpawnBlueprint};
use bevy_butler::system;

use crate::plugins::scene::ScenePlugin;

#[system(schedule = Startup, plugin = ScenePlugin)]
pub fn spawn_test_level(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}
