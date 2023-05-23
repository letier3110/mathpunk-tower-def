use bevy::prelude::*;

pub fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera3dBundle {
        transform: Transform::from_xyz(8.0, 22.5, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
