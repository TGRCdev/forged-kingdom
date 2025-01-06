use bevy::prelude::*;
use bevy_tnua::TnuaUserControlsSystemSet;
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::{components::player::*, systems::player::*, utils::proxy::add_proxy_type};

#[derive(Default, Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Add plugins
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());

        // Register systems
        app.add_systems(
            FixedUpdate,
            player_movement
                .run_if(any_with_component::<Player>)
                .in_set(TnuaUserControlsSystemSet),
        );

        // Register proxies
        add_proxy_type::<proxies::Player>(app);

        // Reflect types
        app.register_type::<proxies::Player>();
    }
}
