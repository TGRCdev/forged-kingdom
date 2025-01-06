use bevy::{
    prelude::*,
    reflect::{GetTypeRegistration, Typed},
};
use blenvy::BlueprintInstanceDisabled;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Proxy<T: Send + Sync>(pub T);

fn replace_proxy_type<T: Bundle + Clone>(
    mut commands: Commands,
    query: Query<(Entity, &Proxy<T>), Without<BlueprintInstanceDisabled>>,
) {
    query.iter().for_each(|(ent, proxy)| {
        commands
            .entity(ent)
            .remove::<Proxy<T>>()
            .insert(proxy.0.clone());
    });
}

pub fn add_proxy_type<T: Bundle + Clone + GetTypeRegistration + FromReflect + Typed>(
    app: &mut App,
) {
    app.register_type::<Proxy<T>>().add_systems(
        Update,
        replace_proxy_type::<T>.run_if(any_with_component::<Proxy<T>>),
    );
}
