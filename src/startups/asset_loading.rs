use bevy::prelude::*;

use crate::structs::game_assets::GameAssets;

pub fn asset_loading(mut cmd: Commands, assets: Res<AssetServer>) {
    let fish = assets.load("high-detailed-fish/source/Fish.glb#Scene0");
    let goblin = assets.load("goblin.glb#Scene0");
    let foundation = assets.load("foundation.glb#Scene0");
    let tower = assets.load("tower.glb#Scene0");
    cmd.insert_resource(GameAssets {
        fish: fish,
        goblin: goblin,
        foundation: foundation,
        tower: tower,
    });
}
