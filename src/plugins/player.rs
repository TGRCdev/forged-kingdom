use bevy::prelude::*;
use bevy_butler::{auto_plugin, configure_plugin};
use bevy_tnua::TnuaUserControlsSystemSet;
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::{components::player::*, systems::player::*, utils::proxy::add_proxy_type};

#[derive(Default, Debug)]
#[auto_plugin]
pub struct PlayerPlugin;

#[configure_plugin(PlayerPlugin)]
fn configure(plugin: &PlayerPlugin, app: &mut App) {
    // Add plugins
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default());

    // Register proxies
    add_proxy_type::<proxies::Player>(app);

    // Reflect types
    app.register_type::<proxies::Player>();
}