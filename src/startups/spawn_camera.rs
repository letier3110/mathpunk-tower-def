use bevy::prelude::*;

pub fn spawn_camera(mut cmd: Commands) {
    cmd.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
