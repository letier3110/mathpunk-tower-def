use bevy::prelude::*;

use crate::structs::health::Health;

pub fn target_death(mut cmd: Commands, targets: Query<(Entity, &Health)>) {
    for (ent, health) in &targets {
        if health.value <= 0 {
            cmd.entity(ent).despawn_recursive();
        }
    }
}
