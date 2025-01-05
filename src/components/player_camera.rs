use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// Spawns a [PlayerCameraPivot] child, which has a [PlayerCamera] child.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct SpawnPlayerCamera {
    pub offset: Vec3,
    pub move_lerp_speed: f32,
    pub rotate_lerp_speed: f32,
    pub zoom_lerp_speed: f32,
    pub cam_dist: f32,
}

impl Default for SpawnPlayerCamera {
    fn default() -> Self {
        Self {
            offset: Vec3::ZERO,
            move_lerp_speed: 1.0,
            rotate_lerp_speed: 1.0,
            zoom_lerp_speed: 1.0,
            cam_dist: 5.0,
        }
    }
}

/// Child entity of a subject that it's following, handles
/// rotation for the child [PlayerCamera].
#[derive(Component, Clone, Debug)]
#[require(Transform)]
#[require(ActionState<PlayerCameraPivotAction>)]
#[require(InputMap<PlayerCameraPivotAction>(|| InputMap::default().with_dual_axis(PlayerCameraPivotAction::Rotate, MouseMove::default())))]
pub struct PlayerCameraPivot {
    pub move_lerp_speed: f32,
    pub rotate_lerp_speed: f32,
}

/// Child entity of a [PlayerCameraPivot], handles zooming.
#[derive(Component, Clone, Debug)]
#[require(Camera3d)]
#[require(ActionState<PlayerCameraAction>)]
#[require(InputMap<PlayerCameraAction>(|| InputMap::default().with_axis(PlayerCameraAction::Zoom, MouseScrollAxis::Y.inverted())))]
pub struct PlayerCamera {
    pub target_dist: f32,
    pub lerp_speed: f32,
}

#[derive(Actionlike, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Reflect, Debug, Hash)]
pub enum PlayerCameraPivotAction {
    #[actionlike(DualAxis)]
    Rotate,
}

#[derive(Actionlike, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Reflect, Debug, Hash)]
pub enum PlayerCameraAction {
    #[actionlike(Axis)]
    Zoom,
}
