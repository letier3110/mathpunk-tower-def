use bevy::prelude::*;

use crate::{
    structs::{bullet::Bullet, lifetime::Lifetime},
    systems::{
        bullet_collision::bullet_collision, bullet_despawn::bullet_despawn,
        move_bullets::move_bullets,
    },
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bullet>()
            .register_type::<Lifetime>()
            .add_system(bullet_collision)
            .add_system(move_bullets)
            .add_system(bullet_despawn);
    }
}
