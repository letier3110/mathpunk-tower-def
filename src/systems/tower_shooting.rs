use bevy::prelude::*;
use bevy::utils::FloatOrd;

use crate::structs::bullet::Bullet;
use crate::structs::game_assets::GameAssets;
use crate::structs::lifetime::Lifetime;
use crate::structs::target::Target;
use crate::structs::tower::Tower;

pub fn tower_shooting(
    mut cmd: Commands,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    fish: Res<GameAssets>,
    time: Res<Time>,
) {
    for (tower_ent, mut tower, transform) in towers.iter_mut() {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn = transform.translation() + tower.bullet_offset;
            cmd.entity(tower_ent).with_children(|commands| {
                let spawn_transform =
                    // Transform::from_translation(bullet_spawn).with_scale(Vec3::splat(8.0));
                    Transform::from_translation(tower.bullet_offset).with_scale(Vec3::splat(8.0));

                let direction = targets
                    .iter()
                    .min_by_key(|target_transform| {
                        FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                    })
                    .map(|closes_target| closes_target.translation() - bullet_spawn);
                // .unwrap_or(Vec3::ZERO);
                if let Some(direction) = direction {
                    commands
                        .spawn(SceneBundle {
                            scene: fish.fish.clone(),
                            transform: spawn_transform,
                            ..default()
                        })
                        .insert(Lifetime {
                            timer: Timer::from_seconds(1000.5, TimerMode::Once),
                        })
                        .insert(Bullet {
                            direction,
                            speed: 2.5,
                        })
                        .insert(Name::new("Bullet"));
                }
            });
        }
    }
}
