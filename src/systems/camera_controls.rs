use bevy::prelude::*;

pub fn camera_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();

    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();

    left.y = 0.0;
    left = left.normalize();

    let speed = 3.0;
    let rotate_speed = 0.2;

    if keyboard_input.pressed(KeyCode::W) {
        camera.translation += forward * speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::S) {
        camera.translation -= forward * speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::A) {
        camera.translation += left * speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::D) {
        camera.translation -= left * speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds());
    }
    if keyboard_input.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds());
    }
}
