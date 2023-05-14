use bevy::prelude::*;

use crate::structs::game_assets::GameAssets;

pub fn asset_loading(mut cmd: Commands, assets: Res<AssetServer>) {
    let scene = assets.load("high-detailed-fish/source/Fish.glb#Scene0");
    cmd.insert_resource(GameAssets { fish: scene });
}
