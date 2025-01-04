use bevy::app::plugin_group;
use super::*;

plugin_group! {
    /// Contains all of the plugins required for the game, excluding [DefaultPlugins](bevy::prelude::DefaultPlugins).
    pub struct GamePlugins
    {
        player_camera:::PlayerCameraPlugin,
        player:::PlayerPlugin,
    }
}