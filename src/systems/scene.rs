use bevy::prelude::*;
use blenvy::{BlueprintInfo, GameWorldTag, HideUntilReady, SpawnBlueprint};

pub fn spawn_test_level(
    mut commands: Commands,
) {
    commands.spawn((
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag
    ));
}