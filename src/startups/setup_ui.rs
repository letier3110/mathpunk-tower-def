use bevy::prelude::*;

use crate::structs::resolution_text::ResolutionText;

pub fn setup_ui(mut cmd: Commands, asset_server: Res<AssetServer>) {
    // Node that fills entire background
    cmd.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|root| {
        // Text where we display current resolution
        root.spawn((
            TextBundle::from_section(
                "Resolution",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 50.0,
                    color: Color::BLACK,
                },
            ),
            ResolutionText,
        ));
    });
}
