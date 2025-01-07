use bevy::prelude::*;
use bevy_butler::auto_plugin;

use crate::systems::scene::*;

#[derive(Default, Debug)]
#[auto_plugin]
pub struct ScenePlugin;