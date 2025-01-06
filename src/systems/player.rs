use bevy::prelude::*;
use bevy_tnua::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::components::player::*;

pub fn player_movement(
    mut query: Query<
        (
            &mut TnuaController,
            &ActionState<PlayerAction>,
            &InputMap<PlayerAction>,
        ),
        With<Player>,
    >,
) {
    query
        .iter_mut()
        .for_each(|(mut controller, action_state, input_map)| {
            if let Some(movement) = action_state
                .dual_axis_data(&PlayerAction::Movement)
                .map(|data| data.update_pair)
            {
                controller.basis(TnuaBuiltinWalk {
                    desired_velocity: Vec3::new(movement.x, 0.0, movement.y),
                    ..default()
                });
            }

            if action_state
                .button_data(&PlayerAction::Jump)
                .is_some_and(|data| data.fixed_update_state.just_pressed())
            {
                controller.action(TnuaBuiltinJump {
                    height: 5.0,
                    ..default()
                });
            }
        });
}
