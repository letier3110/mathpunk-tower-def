use bevy::prelude::*;

use std::f32::consts::PI;

use crate::structs::game_assets::GameAssets;
use crate::structs::lifetime::Lifetime;
use crate::structs::tower::Tower;

pub fn tower_shooting(
    mut cmd: Commands,
    mut towers: Query<&mut Tower>,
    fish: Res<GameAssets>,
    time: Res<Time>,
) {
    for mut tower in towers.iter_mut() {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform = Transform::from_xyz(0.0, 0.7, 0.6)
                .with_rotation(Quat::from_rotation_y(-PI / 2.0))
                .with_scale(Vec3::splat(8.0));
            cmd.spawn(SceneBundle {
                scene: fish.fish.clone(),
                transform: spawn_transform,
                ..default()
            })
            .insert(Lifetime {
                timer: Timer::from_seconds(0.5, TimerMode::Once),
            })
            .insert(Name::new("Bullet"));
        }
    }
}
