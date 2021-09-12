use bevy::{
    prelude::*,
};

use crate::character::{CharacterBundle};
use crate::common_components::Name;

use std::collections::HashMap;

pub struct WorldMap;

#[derive(Debug)]
pub struct WorldMapCamera;

impl WorldMap {
    fn setup(mut commands: Commands)
    {
        commands.spawn()
            .insert_bundle(PerspectiveCameraBundle {
                transform: Transform::from_matrix(
                    Mat4::from_rotation_translation(
                        Quat::from_rotation_x(265f32.to_radians()),
                        Vec3::new(-7.0,80.,4.0),
                    )
                ),
                ..Default::default()
            })
            .insert(WorldMapCamera);
    }
}

impl Plugin for WorldMap
{
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(world_map_movement.system())
            .add_startup_system(Self::setup.system())
            .add_system(camera_rotate.system());
    }
}

#[derive(Debug)]
pub enum WorldMapDestination {
    Position(Vec3),
    Entity(Entity),
    Idle
}

#[derive(Debug, Default)]
pub struct WorldMapSpeed(f32);

impl From<f32> for WorldMapSpeed {
    fn from(speed: f32) -> Self {
        Self(speed)
    }
}

#[derive(Bundle, Default)]
pub struct WorldMapCharacterBundle
{
    #[bundle]
    pub character: CharacterBundle,
    pub transform: Transform,
    pub speed: WorldMapSpeed,
}

fn camera_rotate(time: Res<Time>, mut query: Query<&mut Transform, With<WorldMapCamera>>)
{
    if let Ok(mut trns) = query.single_mut()
    {
        
    }
}

fn move_character_towards(pos: &mut Vec3, dest_pos: Vec3, move_dist: f32, entity: Entity, dest: &mut WorldMapDestination, commands: &mut Commands)
{
    if move_dist*move_dist >= pos.distance_squared(dest_pos)
    {
        *pos = dest_pos;
        *dest = WorldMapDestination::Idle;
    }
    else
    {
        let dir = (dest_pos - *pos).normalize();
        *pos += dir * move_dist;
    }
}

fn world_map_movement(
    time: Res<Time>, 
    mut commands: Commands, 
    mut query : QuerySet<(
        Query<(&Transform, &Name, &WorldMapDestination)>,
        Query<(&mut Transform, &mut WorldMapDestination, &WorldMapSpeed, &Name, Entity)>
    )>,
) {
    let mut pos_table = HashMap::new();
    
    // Step 1. Check for character destinations and pre-fetch the character positions from query 0
    for (_, _, dest) in query.q0().iter()
    {
        if let WorldMapDestination::Entity(id) = dest
        {
            let dest_char = query.q0().get(*id);
            if let Ok((trns, name, _)) = dest_char
            {
                pos_table.insert(*id, (trns.translation, name.0.clone()));
            }
        }
    }

    let pos_table = pos_table;
    let delta = time.delta().as_secs_f32();

    // Step 2. Start mutating with query 1
    for(mut trns, mut dest, speed, name, entity) in query.q1_mut().iter_mut()
    {
        let speed = speed.0;
        let move_dist = speed * delta;
        let pos = &mut trns.translation;
        match *dest
        {
            WorldMapDestination::Position(dest_pos) => {
                move_character_towards(pos, dest_pos, move_dist, entity, &mut dest, &mut commands);
                println!("[{}] ({}) -> ({})", name.0, pos, dest_pos);
            },
            WorldMapDestination::Entity(id) => {
                if let Some((dest_pos, dest_name)) = pos_table.get(&id)
                {
                    move_character_towards(pos, *dest_pos, move_dist, entity, &mut dest, &mut commands);
                    println!("[{}] {} -> [{}]", &name.0, pos, dest_name);
                }
                else
                {
                    *dest = WorldMapDestination::Idle;
                }
            },
            WorldMapDestination::Idle => {}
        }
        
    }
}