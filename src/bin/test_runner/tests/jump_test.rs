use crate::events::{
    CapturePlayerPosition, JumpPlayer, VerifyPlayerIsAtCapturedPosition, VerifyPlayerIsInTheAir,
    WaitStep,
};
use crate::includes::*;
use crate::includes::PlayerCapturedPosition;
use bevy_barcamp::game::player::Player;
use macros::step;
use bevy_barcamp::game::includes::events::PlayerJump;

const MIN_JUMP_HEIGHT: f32 = 50.0;
const LANDING_TOLERANCE: f32 = 10.0;

pub fn provide_steps() -> Vec<Box<dyn TestStep>> {
    vec![
        step!(CapturePlayerPosition),
        step!(JumpPlayer),
        step!(WaitStep {
            updates: 10
        }),
        step!(VerifyPlayerIsInTheAir),
        step!(WaitStep {
            updates:120
        }),
        step!(VerifyPlayerIsAtCapturedPosition),
    ]
}

pub fn handle_verify_player_is_in_the_air(
    _verify_event: On<VerifyPlayerIsInTheAir>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    captured_position: Res<PlayerCapturedPosition>,
    player_query: Query<&Transform, With<Player>>,
) {
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

    unfinished_steps.remove::<VerifyPlayerIsInTheAir>();
    println!("VerifyPlayerIsInTheAir completed.");
}

pub fn handle_verify_player_is_at_captured_position(
    _verify_event: On<VerifyPlayerIsAtCapturedPosition>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    mut captured_position: ResMut<PlayerCapturedPosition>,
    player_query: Query<&Transform, With<Player>>,
) {
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

    unfinished_steps.remove::<VerifyPlayerIsAtCapturedPosition>();
    println!("VerifyPlayerIsAtCapturedPosition completed.");
}

pub fn handle_player_jump(
    _jump_event: On<PlayerJump>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
) {    
    unfinished_steps.remove::<JumpPlayer>();
    println!("JumpPlayer completed.");
}
