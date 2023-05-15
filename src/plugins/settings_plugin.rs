use bevy::prelude::*;

use crate::systems::{
    camera_controls::camera_controls, on_resize_system::on_resize_system,
    toggle_resolution::toggle_resolution,
};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_resize_system)
            .add_system(toggle_resolution)
            .add_system(camera_controls);
    }
}
