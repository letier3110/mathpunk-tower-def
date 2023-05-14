use bevy::prelude::*;

use crate::structs::tower::Tower;
use crate::systems::tower_shooting::tower_shooting;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>().add_system(tower_shooting);
    }
}
