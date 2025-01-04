use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::components::player_camera::PlayerCameraControls;
use crate::systems::player_camera::*;

#[derive(Default, Debug)]
pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerCameraControls>::default())
            .add_systems(PreUpdate, setup_player_camera)
            .add_systems(Update, player_camera_zoom)
            .add_systems(Update, player_camera_rotate);
    }
}
