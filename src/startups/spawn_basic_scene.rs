use bevy::prelude::*;

use crate::structs::health::Health;
use crate::structs::target::Target;
use crate::structs::tower::Tower;

pub fn spawn_basic_scene(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(25.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(Name::new("Floor"));
    cmd.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(Tower {
        shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        bullet_offset: Vec3::new(0.0, 0.7, 0.6),
    })
    .insert(Name::new("Tower"));

    cmd.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
        transform: Transform::from_xyz(-2.0, 0.2, 1.5),
        ..default()
    })
    .insert(Target { speed: 0.3 })
    .insert(Health { value: 3 })
    .insert(Name::new("Target"));

    cmd.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
        transform: Transform::from_xyz(-4.0, 0.2, 1.5),
        ..default()
    })
    .insert(Target { speed: 0.3 })
    .insert(Health { value: 3 })
    .insert(Name::new("Target"));

    cmd.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    })
    .insert(Name::new("Light"));
}
