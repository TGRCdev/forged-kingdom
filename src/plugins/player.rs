use bevy::prelude::*;

use crate::components::player::*;

#[derive(Default, Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Reflect types
        app.register_type::<Player>();
    }
}
