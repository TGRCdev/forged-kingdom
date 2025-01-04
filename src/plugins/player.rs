use bevy::prelude::*;

use crate::systems::player::*;

#[derive(Default, Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreUpdate, setup_player);
    }
}