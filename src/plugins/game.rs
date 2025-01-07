use super::*;
use avian3d::prelude::PhysicsSchedulePlugin;
use bevy::app::{FixedUpdate, PluginGroup, PluginGroupBuilder};

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(bevy_butler::BevyButlerPlugin)
            .add(player::PlayerPlugin)
            .add(player_camera::PlayerCameraPlugin)
            .add_group(avian3d::PhysicsPlugins::default())
            .add(bevy_tnua::prelude::TnuaControllerPlugin::new(FixedUpdate))
            .add_after::<PhysicsSchedulePlugin>(bevy_tnua_avian3d::TnuaAvian3dPlugin::new(
                FixedUpdate,
            ))
            .add(blenvy::BlenvyPlugin::default())
            .add(scene::ScenePlugin)
            .add(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
            .add(collider_proxy::ColliderProxyPlugin)
    }
}
