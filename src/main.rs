mod plugins;
mod startups;
mod structs;
mod systems;

// use startups::*;
use startups::asset_loading::asset_loading;
use startups::setup_ui::setup_ui;
use startups::spawn_basic_scene::spawn_basic_scene;
use startups::spawn_camera::spawn_camera;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowPlugin},
};

use bevy_mod_picking::prelude::*;

use structs::resolution_settings::ResolutionSettings;

use plugins::{
    bullet_plugin::BulletPlugin, settings_plugin::SettingsPlugin, target_plugin::TargetPlugin,
    tower_plugin::TowerPlugin,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                resolution: (500., 300.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .add_startup_system(asset_loading.in_base_set(StartupSet::PreStartup))
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_startup_system(setup_ui)
        .add_plugin(SettingsPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TowerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(BulletPlugin)
        .run();
}
