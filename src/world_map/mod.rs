use bevy::prelude::*;

use crate::{
    core::GameState,
};

#[derive(Default)]
pub struct WorldMapPlugin;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum WorldMapState {
    Paused,
    Running
}

pub struct WorldMapEntity(Entity);

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

fn enter_world_map(mut commands: Commands)
{
    let map = commands.spawn()
    .id();

    commands.insert_resource(WorldMapEntity(map));
}

fn exit_world_map(mut commands: Commands, world_map: Res<WorldMapEntity>)
{
    commands.entity(world_map.0).despawn_recursive();
    commands.remove_resource::<WorldMapEntity>();
}