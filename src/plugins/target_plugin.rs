use bevy::prelude::*;

use crate::{
    structs::{health::Health, target::Target},
    systems::{move_target::move_target, target_death::target_death},
};

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .register_type::<Health>()
            .add_system(move_target)
            .add_system(target_death);
    }
}
