use bevy::prelude::*;

use crate::structs::lifetime::Lifetime;

pub fn bullet_despawn(
    mut cmd: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in bullets.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            cmd.entity(entity).despawn_recursive();
        }
    }
}
