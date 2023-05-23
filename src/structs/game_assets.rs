use bevy::prelude::{Handle, Resource, Scene};

#[derive(Resource)]
pub struct GameAssets {
    pub fish: Handle<Scene>,
    pub goblin: Handle<Scene>,
    pub foundation: Handle<Scene>,
    pub tower: Handle<Scene>,
}
