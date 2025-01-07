use bevy::prelude::*;
use bevy_butler::{auto_plugin, configure_plugin};
use leafwing_input_manager::prelude::*;

use crate::components::player_camera::*;

#[derive(Default, Debug)]
#[auto_plugin]
pub struct PlayerCameraPlugin;

#[configure_plugin(PlayerCameraPlugin)]
fn configure(plugin: &PlayerCameraPlugin, app: &mut App) {
    // Add plugins
    app.add_plugins((
        InputManagerPlugin::<PlayerCameraPivotAction>::default(),
        InputManagerPlugin::<PlayerCameraAction>::default(),
    ));

    // Reflect types
    app.register_type::<SpawnPlayerCamera>();
}
