use bevy::prelude::*;
use bevy_mod_picking::{
    prelude::{RaycastPickCamera, RaycastPickTarget},
    PickableBundle,
};

pub fn spawn_camera(mut cmd: Commands) {
    // cmd.spawn((
    //     PbrBundle::default(),
    //     PickableBundle::default(),
    //     RaycastPickTarget::default(),
    // ));
    cmd.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(8.0, 22.5, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RaycastPickCamera::default(),
    ));
}
