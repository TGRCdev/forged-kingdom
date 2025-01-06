use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::components::player_camera::*;
use crate::systems::player_camera::*;

#[derive(Default, Debug)]
pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        // Add plugins
        app.add_plugins((
            InputManagerPlugin::<PlayerCameraPivotAction>::default(),
            InputManagerPlugin::<PlayerCameraAction>::default(),
        ));

        // Register scheduled systems
        app.add_systems(
            Update,
            (
                spawn_player_camera.run_if(any_with_component::<SpawnPlayerCamera>),
                (
                    player_camera_pivot_handle_rotate_action,
                    player_camera_pivot_lerp_rotation,
                )
                    .chain()
                    .run_if(any_with_component::<PlayerCameraPivot>),
                (player_camera_handle_zoom_action, player_camera_lerp_zoom)
                    .chain()
                    .run_if(any_with_component::<PlayerCamera>),
            ),
        );

        // Reflect types
        app.register_type::<SpawnPlayerCamera>();
    }
}
