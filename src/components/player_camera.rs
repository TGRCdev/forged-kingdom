use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// Spawns a [PlayerCamera] entity with this entity as the subject.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct SpawnPlayerCamera {
    pub offset: Vec3,
    pub move_lerp_speed: f32,
    pub rotate_lerp_speed: f32,
    pub zoom: f32,
}

impl Default for SpawnPlayerCamera {
    fn default() -> Self {
        Self {
            offset: Vec3::ZERO,
            move_lerp_speed: 1.0,
            rotate_lerp_speed: 1.0,
            zoom: 5.0,
        }
    }
}

#[derive(Component, Clone, Debug)]
#[require(Camera3d)]
pub struct PlayerCamera {
    pub subject: Entity,
    pub offset: Vec3,
    pub zoom: f32,
    pub move_lerp_speed: f32,
    pub rotate_lerp_speed: f32,
}

#[derive(Component, Clone, Debug)]
#[require(Transform)]
pub struct PlayerCameraSubject(pub u32);

#[derive(Actionlike, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Reflect, Debug, Hash)]
pub enum PlayerCameraControls {
    #[actionlike(DualAxis)]
    Rotate,
    #[actionlike(Axis)]
    Zoom,
}
