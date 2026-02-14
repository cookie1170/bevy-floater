use bevy::prelude::*;

/// A plugin that makes [`Controller`](crate::controller::Controller)s work
pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, crate::controller::update_controllers);
    }
}
