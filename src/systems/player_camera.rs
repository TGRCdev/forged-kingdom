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
                    target_rotation: Quat::IDENTITY,
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

/// Adjust the [PlayerCameraPivot]
pub fn player_camera_pivot_handle_rotate_action(
    mut cam_query: Query<(
        &mut PlayerCameraPivot,
        &ActionState<PlayerCameraPivotAction>,
    )>,
) {
    cam_query.iter_mut().for_each(|(mut cam, action_state)| {
        if let Some(rotation) = action_state
            .dual_axis_data(&PlayerCameraPivotAction::Rotate)
            .and_then(|data| {
                Some(data.update_pair).filter(|val| val.is_finite() && *val != Vec2::ZERO)
            })
        {
            // Rotate around Y axis
            cam.target_rotation =
                Quat::from_axis_angle(Vec3::Y, -rotation.x.to_radians()) * cam.target_rotation;

            // Rotate around local X axis
            cam.target_rotation *= Quat::from_axis_angle(Vec3::X, -rotation.y.to_radians());
        }
    });
}

/// Every frame, lerp the camera pivot's rotation towards the target rotation
pub fn player_camera_pivot_lerp_rotation(
    mut cam_query: Query<(&mut Transform, &PlayerCameraPivot)>,
) {
    cam_query.iter_mut().for_each(|(mut xform, cam)| {
        xform.rotation = xform
            .rotation
            .lerp(cam.target_rotation, cam.rotate_lerp_speed.clamp(0.0, 1.0));
    });
}

/// Adjust the camera's target zoom level when the zoom is changed.
pub fn player_camera_handle_zoom_action(
    mut cam_query: Query<(&mut PlayerCamera, &ActionState<PlayerCameraAction>)>,
) {
    cam_query.iter_mut().for_each(|(mut cam, action_state)| {
        if let Some(zoom_val) = action_state
            .axis_data(&PlayerCameraAction::Zoom)
            .and_then(|data| Some(data.update_value).filter(|val| val.is_normal()))
        {
            cam.target_dist += zoom_val;
        }
    });
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
