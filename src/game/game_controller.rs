use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;

use super::{
    camera::CameraPlugin,
    events::{QuitGame, StartGame},
    interaction::InteractionPlugin,
    player::PlayerPlugin,
    resources::UnfinishedStateTransitions,
    state::GameState,
    terrain::TerrainPlugin,
};

#[derive(Default)]
pub struct GameControllerPlugin;

impl Plugin for GameControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UnfinishedStateTransitions>()
            .add_plugins((TerrainPlugin, CameraPlugin, PlayerPlugin, InteractionPlugin))
            .add_systems(Update, advance_state)
            .add_observer(handle_start_game)
            .add_observer(handle_quit_game);
    }
}

fn handle_start_game(
    _start: On<StartGame>,
    state: Res<State<GameState>>,
    transitions: Res<UnfinishedStateTransitions>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *state.get() == GameState::Uninitialized && transitions.is_empty() {
        next_state.set(GameState::Initializing);
    }
}

fn handle_quit_game(
    _quit: On<QuitGame>,
    state: Res<State<GameState>>,
    transitions: Res<UnfinishedStateTransitions>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *state.get() == GameState::Running && transitions.is_empty() {
        next_state.set(GameState::Quitting);
    }
}

fn advance_state(
    state: Res<State<GameState>>,
    transitions: Res<UnfinishedStateTransitions>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !transitions.is_empty() {
        return;
    }

    let current = *state.get();
    match current {
        GameState::Initializing | GameState::Quitting => next_state.set(current.next()),
        GameState::Uninitialized | GameState::Running => {}
    }
}

