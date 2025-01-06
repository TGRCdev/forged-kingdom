use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::controller::TnuaController;
use leafwing_input_manager::prelude::*;

pub mod proxies {
    use bevy::prelude::*;

    #[derive(Component, Reflect, Default, Debug, Clone, Copy)]
    #[reflect(Component)]
    pub struct Player;
}

#[derive(Component, Default, Debug)]
#[require(Transform, TnuaController)]
#[require(LockedAxes(|| LockedAxes::ROTATION_LOCKED))]
#[require(ActionState<PlayerAction>)]
#[require(InputMap<PlayerAction>(|| {
    InputMap::new([(PlayerAction::Jump, KeyCode::Space)])
        .with_dual_axis(PlayerAction::Movement, VirtualDPad::new(KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD))
}))]
pub struct Player;

#[derive(Actionlike, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Reflect, Debug, Hash)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Movement,
    #[actionlike(Button)]
    Jump,
}
