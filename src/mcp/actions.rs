use bevy::prelude::*;
use crate::game::includes::events::{Direction, PlayerJump, PlayerMove, QuitGame, StartGame};
use crate::game::includes::state::GameState;
use crate::game::mob::Mob;
use crate::game::player::Player;
use crate::game::terrain::TerrainPiece;

/// Remote action queue that BRP clients can populate to trigger gameplay events.
#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct McpActionQueue {
    pub actions: Vec<McpAction>,
}

/// Latest world information snapshot populated by MCP query actions.
#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct McpWorldState {
    pub player_position: Option<Vec3>,
    pub game_bounds: Option<McpGameBounds>,
    pub platforms: Vec<Vec3>,
    pub mob_position: Option<Vec3>,
    pub game_state: Option<String>,
}

#[derive(Reflect, Clone, Debug, Default)]
pub struct McpGameBounds {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

#[derive(Reflect, Clone, Debug)]
pub enum McpAction {
    StartGame,
    StopGame,
    Move(McpMoveAction),
    Jump,
    GetGameStatus,
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

pub fn consume_actions(
    mut queue: ResMut<McpActionQueue>,
    mut commands: Commands,
    mut world_state: ResMut<McpWorldState>,
    player_query: Query<&Transform, With<Player>>,
    terrain_query: Query<(&Transform, &Sprite, Option<&Name>), With<TerrainPiece>>,
    mob_query: Query<&Transform, With<Mob>>,
    game_state: Res<State<GameState>>,
) {
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
            McpAction::GetGameStatus => {
                world_state.player_position =
                    player_query.iter().next().map(|transform| transform.translation);
                world_state.game_bounds = compute_game_bounds(&terrain_query);
                world_state.platforms = collect_platform_positions(&terrain_query);
                world_state.mob_position =
                    mob_query.iter().next().map(|transform| transform.translation);
                world_state.game_state = Some(format!("{:?}", game_state.get()));
            }
        }
    }
}

fn compute_game_bounds(
    terrain_query: &Query<(&Transform, &Sprite, Option<&Name>), With<TerrainPiece>>,
) -> Option<McpGameBounds> {
    terrain_query
        .iter()
        .find(|(_, _, name)| name.map(|n| n.as_str()) == Some("Ground"))
        .and_then(|(transform, sprite, _)| {
            sprite.custom_size.map(|size| {
                let half_size = size / 2.0;
                let center = transform.translation;
                McpGameBounds {
                    left: center.x - half_size.x,
                    right: center.x + half_size.x,
                    bottom: center.y - half_size.y,
                    top: center.y + half_size.y,
                }
            })
        })
}

fn collect_platform_positions(
    terrain_query: &Query<(&Transform, &Sprite, Option<&Name>), With<TerrainPiece>>,
) -> Vec<Vec3> {
    terrain_query
        .iter()
        .filter(|(_, _, name)| {
            name.map(|n| n.as_str().starts_with("Platform")).unwrap_or(false)
        })
        .map(|(transform, _, _)| transform.translation)
        .collect()
}
