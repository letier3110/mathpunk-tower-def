use bevy::prelude::{Handle, Resource, Scene};

#[derive(Resource)]
pub struct GameAssets {
    pub fish: Handle<Scene>,
}
