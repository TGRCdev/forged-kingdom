use bevy::prelude::*;

use crate::common_components::Name;

#[derive(Default)]
pub struct Character;

#[derive(Bundle, Default)]
pub struct CharacterBundle
{
    pub character: Character,
    pub name: Name,
}