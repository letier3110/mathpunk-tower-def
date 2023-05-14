use bevy::prelude::{Query, Res, Time, Transform};

use crate::structs::bullet::Bullet;

pub fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in bullets.iter_mut() {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
    }
}
