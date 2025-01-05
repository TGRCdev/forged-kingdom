use bevy::prelude::*;

use crate::{components::player::*, systems::player::player_movement};

#[derive(Default, Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Add plugins

        // Register systems
        app.add_systems(Update, player_movement);

        // Reflect types
        app.register_type::<Player>();
    }
}
