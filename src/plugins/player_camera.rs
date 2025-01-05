use bevy::prelude::*;
use blenvy::GltfBlueprintsSet;
use leafwing_input_manager::prelude::*;

use crate::components::player_camera::*;
use crate::systems::player_camera::*;

#[derive(Default, Debug)]
pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        // Add plugins
        app.add_plugins(InputManagerPlugin::<PlayerCameraControls>::default());

        // Register systems
        app.add_systems(Update, spawn_player_camera)
            .add_systems(Update, player_camera_subject_check_refcount)
            .add_systems(Update, (rotate_player_camera, move_player_camera).chain());

        // Reflect types
        app.register_type::<SpawnPlayerCamera>()
            .register_type::<Vec3>();
    }
}
