use bevy::{prelude::*, utils::HashMap};

use crate::components::player::Player;

pub fn player_movement(
    mut query: Query<(Entity, &mut Transform), With<Player>>,
    time: Res<Time>,
    mut offsets: Local<HashMap<Entity, Vec3>>,
) {
    for (player, mut xform) in &mut query {
        let original_offset = *offsets.entry(player).or_insert_with(|| xform.translation);

        let offset = Vec3::new(time.elapsed_secs().sin(), time.elapsed_secs().cos(), 0.0);

        xform.translation = original_offset + offset;
    }
}
