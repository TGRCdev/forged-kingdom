use bevy::prelude::*;
use bevy_butler::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::{components::player::*, utils::proxy::add_proxy_type};

#[derive(Default, Debug)]
pub struct PlayerPlugin;

#[butler_plugin]
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Add plugins
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());

        // Register proxies
        add_proxy_type::<proxies::Player>(app);

        // Reflect types
        app.register_type::<proxies::Player>();
    }
}