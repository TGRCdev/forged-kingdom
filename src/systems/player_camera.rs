use bevy::prelude::*;
use blenvy::BlueprintInstanceDisabled;
use leafwing_input_manager::prelude::*;

use crate::components::player_camera::*;

/// Remove [SpawnPlayerCamera] and spawn a child entity with [PlayerCameraPivot],
/// which has a child entity with [PlayerCamera].
pub fn spawn_player_camera(
    mut commands: Commands,
    mut query: Query<(Entity, &SpawnPlayerCamera), Without<BlueprintInstanceDisabled>>,
) {
    for (subject, args) in &mut query {
        let pivot_id = commands
            .spawn((
                PlayerCameraPivot {
                    move_lerp_speed: args.move_lerp_speed,
                    rotate_lerp_speed: args.rotate_lerp_speed,
                },
                Transform::from_translation(args.offset),
            ))
            .with_child((
                PlayerCamera {
                    target_dist: args.cam_dist,
                    lerp_speed: args.zoom_lerp_speed,
                },
                Transform::from_xyz(0.0, 0.0, args.cam_dist),
            ))
            .id();

        info!("Spawning PlayerCamera with pivot {pivot_id} on subject {subject}");

        commands
            .entity(subject)
            .remove::<SpawnPlayerCamera>()
            .add_child(pivot_id);
    }
}

/// Rotate the player camera's pivot
pub fn rotate_player_camera_pivot(
    mut cam_query: Query<
        (&mut Transform, &ActionState<PlayerCameraPivotAction>),
        With<PlayerCameraPivot>,
    >,
) {
    for (mut cam_xform, action_state) in cam_query.iter_mut() {
        match action_state
            .dual_axis_data(&PlayerCameraPivotAction::Rotate)
            .map(|data| data.update_pair)
        {
            None | Some(Vec2::ZERO) => (),
            Some(rotation) => {
                if rotation.is_finite() {
                    cam_xform.rotate_y(-rotation.x.to_radians());
                    cam_xform.rotate_local_x(-rotation.y.to_radians());
                }
            }
        }
    }
}

/// Adjust the camera's target zoom level when the zoom is changed.
pub fn player_camera_handle_zoom_action(
    mut cam_query: Query<(&mut PlayerCamera, &ActionState<PlayerCameraAction>)>,
) {
    for (mut cam, action_state) in &mut cam_query {
        if let Some(zoom_val) = action_state
            .axis_data(&PlayerCameraAction::Zoom)
            .and_then(|data| {
                if data.update_value.is_normal() {
                    Some(data.update_value)
                } else {
                    None
                }
            })
        {
            cam.target_dist += zoom_val;
        }
    }
}

/// Every frame, lerp the camera's current Z coordinate to the target
/// zoom level.
pub fn player_camera_lerp_zoom(mut cam_query: Query<(&PlayerCamera, &mut Transform)>) {
    cam_query.iter_mut().for_each(|(cam, mut xform)| {
        xform.translation.z = xform
            .translation
            .z
            .lerp(cam.target_dist, cam.lerp_speed.clamp(0.0, 1.0));
    });
}
