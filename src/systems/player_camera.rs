use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::components::player_camera::*;

/// Remove instances of [SpawnPlayerCamera] and spawn an entity with [PlayerCamera],
/// with their subject set to the entity with [SpawnPlayerCamera].
pub fn spawn_player_camera(
    mut commands: Commands,
    mut query: Query<
        (Entity, Option<&mut PlayerCameraSubject>, &SpawnPlayerCamera),
        Added<SpawnPlayerCamera>,
    >,
) {
    for (subject, comp, args) in query.iter_mut() {
        commands.spawn(PlayerCamera {
            subject,
            offset: Vec3::ZERO,
            move_lerp_speed: args.move_lerp_speed,
            rotate_lerp_speed: args.rotate_lerp_speed,
            zoom: args.zoom,
        });

        commands.entity(subject).remove::<SpawnPlayerCamera>();

        if let Some(mut comp) = comp {
            // Entity is already a subject of another player camera, increment
            // the reference counter
            comp.0 += 1;
        } else {
            // Entity needs a new ref counter
            commands.entity(subject).insert(PlayerCameraSubject(1));
        }
    }
}

/// If a [PlayerCameraSubject]'s refcount goes to 0, remove the component.
pub fn player_camera_subject_check_refcount(
    mut commands: Commands,
    query: Query<(Entity, &PlayerCameraSubject), Changed<PlayerCameraSubject>>,
) {
    for (subject, _) in query.iter().filter(|(_, comp)| comp.0 == 0) {
        commands.entity(subject).remove::<PlayerCameraSubject>();
    }
}

pub fn move_player_camera(
    mut query_set: ParamSet<(
        Query<(Entity, &Transform), With<PlayerCameraSubject>>,
        Query<(&mut Transform, &PlayerCamera)>,
    )>,
) {
    let offsets: HashMap<Entity, Vec3> = query_set
        .p0()
        .iter()
        .map(|(subject, xform)| (subject, xform.translation))
        .collect();

    for (mut xform, pcam) in query_set.p1().iter_mut() {
        let target_offset = offsets[&pcam.subject] + (xform.back() * pcam.zoom) + pcam.offset;
        xform.translation = xform
            .translation
            .lerp(target_offset, pcam.move_lerp_speed.clamp(0.0, 1.0));
    }
}
