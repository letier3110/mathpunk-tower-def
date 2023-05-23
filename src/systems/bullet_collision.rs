use bevy::prelude::*;

use crate::structs::{bullet::Bullet, health::Health, target::Target};

pub fn bullet_collision(
    mut cmd: Commands,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
    mut targets: Query<(&mut Health, &Transform), With<Target>>,
) {
    for (bullet, bullet_transform) in bullets.iter() {
        for (mut health, target_transform) in targets.iter_mut() {
            let distance =
                Vec3::distance(target_transform.translation, bullet_transform.translation());
            if distance < 1.5 {
                health.value -= 1;
                cmd.entity(bullet).despawn_recursive();
                break;
            }
        }
    }
}

// check Heron package
