use avian3d::prelude::*;
use bevy::prelude::*;

mod components;
use camera::{PlayerCamera, PlayerCameraPlugin};
pub use components::*;

pub mod camera;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreUpdate, setup_player)
            .add_plugins(PlayerCameraPlugin);
    }
}

pub fn setup_player(
    mut commands: Commands,
    query: Query<Entity, Added<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut main_player: Local<Option<Entity>>,
)
{
    for ent in query.iter()
    {
        if let Some(player) = *main_player
        {
            panic!("Tried to initialize a second player {} (Existing player {})", ent, player);
        }

        info!("Setting up player {}", ent);

        let player_camera = commands
            .spawn((
                PlayerCamera::default(),
                Transform::from_xyz(0.0, 0.5, 0.0)
            ))
            .id();

        commands.entity(ent)
            .insert(Mesh3d(meshes.add(Capsule3d::new(0.5,2.0))))
            .insert(MeshMaterial3d(materials.add(Color::linear_rgb(0.0,0.0,1.0))))
            .insert(RigidBody::Dynamic)
            .insert(Collider::capsule(0.5, 2.0))
            .add_child(player_camera);
        
        *main_player = Some(ent);
    }
}