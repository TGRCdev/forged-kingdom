use bevy::prelude::*;
use crate::{
    core::GameState,
    world_map::WorldMapState
};

#[derive(Default)]
pub struct WorldMapPlugin;
#[derive(Default)]
pub struct WorldMapSingleton;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(WorldMapState::Paused)
            .add_system_set(
                SystemSet::on_enter(GameState::WorldMap)
                .with_system(enter_world_map.system())
            )
            .add_system_set(
                SystemSet::on_exit(GameState::WorldMap)
                .with_system(exit_world_map.system())
            );
    }
}

fn enter_world_map(mut commands: Commands, query: Query<Entity, With<WorldMapSingleton>>)
{
    if let Ok(map) = query.single()
    {
        error!("WorldMapSingleton is already present??? (Entity: {:?})", map);
    }
    else {
        commands.spawn()
        .insert(WorldMapSingleton)
        .id();
    }
}

fn exit_world_map(mut commands: Commands, mut query: Query<Entity, With<WorldMapSingleton>>)
{
    if let Ok(map_entity) = query.single_mut()
    {
        commands.entity(map_entity).despawn_recursive();
    }
    else
    {
        warn!("WARN: While exiting WorldMap state, no WorldMapSingleton entity was found");
    }
}