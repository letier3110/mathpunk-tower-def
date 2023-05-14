use bevy::prelude::{Query, Res, Time, Transform};

use crate::structs::target::Target;

pub fn move_target(mut targets: Query<(&Target, &mut Transform)>, time: Res<Time>) {
    for (target, mut transform) in targets.iter_mut() {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}
