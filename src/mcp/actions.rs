use bevy::prelude::*;

use crate::game::includes::events::{Direction, PlayerJump, PlayerMove, QuitGame, StartGame};

/// Remote action queue that BRP clients can populate to trigger gameplay events.
#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct McpActionQueue {
    pub actions: Vec<McpAction>,
}

#[derive(Reflect, Clone, Debug)]
pub enum McpAction {
    StartGame,
    StopGame,
    Move(McpMoveAction),
    Jump,
}

#[derive(Reflect, Clone, Debug)]
pub struct McpMoveAction {
    pub direction: McpMoveDirection,
    pub state: McpButtonState,
}

#[derive(Reflect, Clone, Copy, Debug)]
pub enum McpMoveDirection {
    Left,
    Right,
}

#[derive(Reflect, Clone, Copy, Debug)]
pub enum McpButtonState {
    Pressed,
    Released,
}

pub fn consume_actions(mut queue: ResMut<McpActionQueue>, mut commands: Commands) {
    if queue.actions.is_empty() {
        return;
    }

    for action in queue.actions.drain(..) {
        match action {
            McpAction::StartGame => commands.trigger(StartGame),
            McpAction::StopGame => commands.trigger(QuitGame),
            McpAction::Jump => commands.trigger(PlayerJump),
            McpAction::Move(move_action) => {
                let direction = match move_action.direction {
                    McpMoveDirection::Left => Direction::Left,
                    McpMoveDirection::Right => Direction::Right,
                };

                let active = matches!(move_action.state, McpButtonState::Pressed);
                commands.trigger(PlayerMove { direction, active });
            }
        }
    }
}
