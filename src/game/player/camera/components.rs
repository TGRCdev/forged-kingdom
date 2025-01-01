use bevy::prelude::*;

#[derive(Component, Default, Debug)]
#[require(Camera3d)]
pub struct PlayerCamera
{
    pub pivot: Option<Entity>
}

#[derive(Component, Default, Debug)]
#[require(Transform)]
pub struct PlayerCameraPivot;