use super::{
    camera::CameraPlugin, interaction::InteractionPlugin, mob::MobPlugin, player::PlayerPlugin,
    terrain::TerrainPlugin,
};
use crate::game::includes::events::{QuitGame, StartGame};
use crate::game::includes::resources::{TargetState, UnfinishedStateTransitions};
use crate::game::includes::state::GameState;
use bevy::prelude::*;

#[derive(Default)]
pub struct GameControllerPlugin;

impl Plugin for GameControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UnfinishedStateTransitions>()
            .init_resource::<TargetState>()
            .add_plugins((
                TerrainPlugin,
                CameraPlugin,
                PlayerPlugin,
                MobPlugin,
                InteractionPlugin,
            ))
            .add_systems(Update, advance_state.run_if(target_state_requested))
            .add_observer(handle_start_game)
            .add_observer(handle_quit_game);
    }
}

fn handle_start_game(_start: On<StartGame>, mut target: ResMut<TargetState>) {
    target.state = Some(GameState::Running);
}

fn handle_quit_game(_quit: On<QuitGame>, mut target: ResMut<TargetState>) {
    target.state = Some(GameState::Uninitialized);
}

fn advance_state(
    state: Res<State<GameState>>,
    mut target_state_res: ResMut<TargetState>,
    transitions: Res<UnfinishedStateTransitions>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(target_state) = target_state_res.state else {
        return;
    };

    let current_state = *state.get();
    if current_state == target_state {
        target_state_res.state = None;
        return;
    }

    if transitions.count() == 0 {
        next_state.set(current_state.next());
    }
}

fn target_state_requested(target: Res<TargetState>) -> bool {
    target.state.is_some()
}
