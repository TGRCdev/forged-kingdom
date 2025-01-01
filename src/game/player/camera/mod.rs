use bevy::prelude::*;

mod components;
pub use components::*;
use leafwing_input_manager::prelude::*;

use super::setup_player;

pub struct PlayerCameraPlugin;

#[derive(Actionlike, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Reflect, Debug, Hash)]
pub enum CameraControls {
    #[actionlike(DualAxis)]
    Rotate,
    #[actionlike(Axis)]
    Zoom,
}

impl Plugin for PlayerCameraPlugin
{
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<CameraControls>::default())
            .add_systems(PreUpdate, setup_player_camera.after(setup_player))
            .add_systems(Update, player_camera_zoom)
            .add_systems(Update, player_camera_rotate);
    }
}

fn setup_player_camera(
    mut commands: Commands,
    mut query: Query<(Entity, &Parent, &mut PlayerCamera, &mut Transform), Added<PlayerCamera>>,
)
{
    info_once!("setup_player_camera");
    for (newcam, parent, mut camera, mut transform) in query.iter_mut()
    {
        info!("Setting up camera entity {}", newcam);

        let input_map = InputMap::default()
            .with_dual_axis(CameraControls::Rotate, MouseMove::default())
            .with_axis(CameraControls::Zoom, MouseScrollAxis::Y);

        commands.entity(parent.get())
            .remove_children(&[newcam]);

        let pivot = commands.spawn(PlayerCameraPivot)
            .insert(transform.clone())
            .add_child(newcam)
            .id();

        commands.entity(parent.get()).add_child(pivot);

        camera.pivot = Some(pivot);
        commands.entity(newcam).insert(InputManagerBundle::with_map(input_map));
        transform.translation = Vec3::new(0.0, 0.0, 10.0);
    }
}

fn player_camera_rotate(
    cam_query: Query<(&ActionState<CameraControls>, &PlayerCamera)>,
    mut pivot_query: Query<&mut Transform, With<PlayerCameraPivot>>
)
{
    for (action_state, camera) in cam_query.iter()
    {
        let rotation = action_state.dual_axis_data(&CameraControls::Rotate).map(|axis| axis.update_pair);
        if rotation.is_none()
            { continue; }
        let rotation = -rotation.unwrap();

        let pivot = camera.pivot.and_then(|ent| pivot_query.get_mut(ent).ok());
        if pivot.is_none()
            { continue; }
        let mut pivot = pivot.unwrap();

        pivot.rotate_local_x(rotation.y.to_radians());
        pivot.rotate_y(rotation.x.to_radians());
    }
}

fn player_camera_zoom(
    mut cam_query: Query<(&mut Transform, &ActionState<CameraControls>), With<PlayerCamera>>,
)
{
    for (mut xform, action_state) in cam_query.iter_mut()
    {
        let zoom_val = -action_state.value(&CameraControls::Zoom);
        if zoom_val != 0.0
            { info!("Zoom: {}", zoom_val); }
        xform.translation.z += zoom_val;
    }
}