use crate::events::{
    CapturePlayerPosition, JumpPlayer, VerifyPlayerIsAtCapturedPosition, VerifyPlayerIsInTheAir,
    WaitStep,
};
use crate::includes::*;
use crate::includes::PlayerCapturedPosition;
use bevy::prelude::*;
use bevy_barcamp::game::includes::state::GameState;
use bevy_barcamp::game::player::Player;
use macros::step;

const MIN_JUMP_HEIGHT: f32 = 50.0;
const LANDING_TOLERANCE: f32 = 10.0;
const WAIT_FOR_ASCENT_UPDATES: usize = 12;
const WAIT_FOR_LANDING_UPDATES: usize = 36;

pub fn provide_steps() -> Vec<Box<dyn TestStep>> {
    vec![
        step!(CapturePlayerPosition),
        step!(JumpPlayer),
        step!(WaitStep {
            updates: WAIT_FOR_ASCENT_UPDATES
        }),
        step!(VerifyPlayerIsInTheAir),
        step!(WaitStep {
            updates: WAIT_FOR_LANDING_UPDATES
        }),
        step!(VerifyPlayerIsAtCapturedPosition),
    ]
}

pub fn handle_jump_player(
    _jump_event: On<JumpPlayer>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    game_state: Res<State<GameState>>,
    mut _commands: Commands,
) {
    println!("Handling JumpPlayer");

    assert_eq!(
        *game_state.get(),
        GameState::Running,
        "JumpPlayer triggered outside of GameState::Running"
    );

    // The actual PlayerJump event will be dispatched here once implemented in the game code.

    unfinished_steps.sub_one();
    println!("JumpPlayer completed.");
}

pub fn handle_verify_player_is_in_the_air(
    _verify_event: On<VerifyPlayerIsInTheAir>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    captured_position: Res<PlayerCapturedPosition>,
    player_query: Query<&Transform, With<Player>>,
    game_state: Res<State<GameState>>,
) {
    println!("Handling VerifyPlayerIsInTheAir");

    assert_eq!(
        *game_state.get(),
        GameState::Running,
        "VerifyPlayerIsInTheAir triggered outside of GameState::Running"
    );

    let baseline_position = captured_position
        .0
        .expect("Player baseline position missing before verifying airborne state");

    let player_transform = player_query
        .iter()
        .next()
        .expect("Player entity not found when verifying airborne state");
    let current_position = player_transform.translation;

    assert!(
        current_position.y > baseline_position.y + MIN_JUMP_HEIGHT,
        "Expected player to be airborne by at least {MIN_JUMP_HEIGHT}, but y went from {} to {}",
        baseline_position.y,
        current_position.y
    );

    unfinished_steps.sub_one();
    println!("VerifyPlayerIsInTheAir completed.");
}

pub fn handle_verify_player_is_at_captured_position(
    _verify_event: On<VerifyPlayerIsAtCapturedPosition>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    mut captured_position: ResMut<PlayerCapturedPosition>,
    player_query: Query<&Transform, With<Player>>,
    game_state: Res<State<GameState>>,
) {
    println!("Handling VerifyPlayerIsAtCapturedPosition");

    assert_eq!(
        *game_state.get(),
        GameState::Running,
        "VerifyPlayerIsAtCapturedPosition triggered outside of GameState::Running"
    );

    let baseline_position = captured_position
        .0
        .expect("Player baseline position missing before verifying landing state");

    let player_transform = player_query
        .iter()
        .next()
        .expect("Player entity not found when verifying landing state");
    let current_position = player_transform.translation;

    assert!(
        (current_position.y - baseline_position.y).abs() <= LANDING_TOLERANCE,
        "Expected player to land within {LANDING_TOLERANCE} of baseline y {}, but current y is {}",
        baseline_position.y,
        current_position.y
    );

    captured_position.0 = Some(current_position);

    unfinished_steps.sub_one();
    println!("VerifyPlayerIsAtCapturedPosition completed.");
}
