use bevy::prelude::*;
use bevy_butler::*;
use leafwing_input_manager::prelude::*;

use crate::components::player_camera::*;

#[derive(Default, Debug)]
pub struct PlayerCameraPlugin;

#[butler_plugin]
impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        // Add plugins
        app.add_plugins((
            InputManagerPlugin::<PlayerCameraPivotAction>::default(),
            InputManagerPlugin::<PlayerCameraAction>::default(),
        ));

        // Reflect types
        app.register_type::<SpawnPlayerCamera>();
    }
}
