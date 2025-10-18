mod actions;

use bevy::prelude::*;
use bevy_brp_extras::BrpExtrasPlugin;

pub use actions::{
    consume_actions, McpAction, McpActionQueue, McpButtonState, McpMoveAction, McpMoveDirection,
};

#[derive(Default)]
pub struct McpPlugin;

impl Plugin for McpPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BrpExtrasPlugin)
            .register_type::<McpActionQueue>()
            .register_type::<McpAction>()
            .register_type::<McpMoveAction>()
            .register_type::<McpMoveDirection>()
            .register_type::<McpButtonState>()
            .init_resource::<McpActionQueue>()
            .add_systems(Update, consume_actions);
    }
}
