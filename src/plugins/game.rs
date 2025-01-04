use super::*;
use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct GamePlugins;

impl PluginGroup for GamePlugins
{
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<GamePlugins>()
            .add(player::PlayerPlugin)
            .add(player_camera::PlayerCameraPlugin)
            .add_group(avian3d::PhysicsPlugins::default())
            .add(blenvy::BlenvyPlugin::default())
            .add(scene::ScenePlugin)
    }
}