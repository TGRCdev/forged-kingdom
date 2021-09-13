use bevy::prelude::*;
use crate::world_map::WorldMapPlugin;

#[derive(Default)]
struct CorePlugin;

impl Plugin for CorePlugin
{
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::MainMenu);
    }
}

#[derive(Default)]
pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(CorePlugin)
            .add(WorldMapPlugin);
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    Paused,
    WorldMap,
    RoamArea,
}