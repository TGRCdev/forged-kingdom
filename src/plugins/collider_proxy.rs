use avian3d::prelude::ColliderConstructor;
use bevy::prelude::*;

use crate::utils::proxy::add_proxy_type;

pub struct ColliderProxyPlugin;

impl Plugin for ColliderProxyPlugin {
    fn build(&self, app: &mut App) {
        add_proxy_type::<ColliderConstructor>(app);
    }
}
