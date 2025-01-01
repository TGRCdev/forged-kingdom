use bevy::prelude::*;
use avian3d::prelude::*;

use crate::game::Player;

pub fn add_test_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{
    // Floor
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(15.0,0.5,15.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -1.0, 0.0),
        Collider::cuboid(15.0, 0.5, 15.0),
        RigidBody::Static,
    ));

    // Player
    commands.spawn((
        Player,
        Transform::from_xyz(1.0, 2.0, 1.0)
    ));
}