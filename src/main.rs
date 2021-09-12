use bevy::{input::system::exit_on_esc_system, pbr::AmbientLight, prelude::{*, shape::Cube}, render::color::Color};

mod world_map;
mod character;
mod common_components;

use character::{CharacterBundle, Character};
use world_map::{WorldMap, WorldMapCharacterBundle, WorldMapDestination};
use common_components::Name;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>)
{
    let char = CharacterBundle {
        character: Character,
        name: Name("Butterlord of Swadia".to_string()),
    };
    let cube = meshes.add(Cube::default().into());
    let butterlord = commands.spawn()
        .insert_bundle(PbrBundle {
            mesh: cube.clone(),
            material: materials.add(Color::rgb(0.7,0.3,0.).into()),
            ..Default::default()
        })
        .insert_bundle(WorldMapCharacterBundle {
            character: char,
            speed: 2.0.into(),
            ..Default::default()
        }).insert(WorldMapDestination::Position(Vec3::new(30.0, 0.0, 5.0)))
        .id();
    commands.spawn()
        .insert_bundle(PbrBundle {
            mesh: cube,
            material: materials.add(Color::rgb(0.7,0.,0.).into()),
            ..Default::default()
        })
        .insert_bundle(WorldMapCharacterBundle {
            character: CharacterBundle { character: Character, name: Name("Flaxlord of Rhodok".to_string())},
            speed: 1.7.into(),
            transform: Transform::from_translation(Vec3::new(-15.0, 0.0, 20.0)),
            ..Default::default()
        })
        .insert(WorldMapDestination::Entity(butterlord));
    
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
}

fn main()
{
    App::build()
    .insert_resource(WindowDescriptor {
        title: "Forged Kingdom".to_string(),
        ..Default::default()
    })
    .add_startup_system(setup.system())
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldMap)
    .add_system(exit_on_esc_system.system())
    .run();
}