use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component, Reflect, Clone, Default, Debug)]
#[require(Camera3d)]
#[reflect(opaque)]
pub struct PlayerCamera {
    pub pivot: Option<Entity>,
}

#[derive(Component, Default, Debug)]
#[require(Transform)]
pub struct PlayerCameraPivot;

#[derive(Actionlike, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Reflect, Debug, Hash)]
pub enum PlayerCameraControls {
    #[actionlike(DualAxis)]
    Rotate,
    #[actionlike(Axis)]
    Zoom,
}
