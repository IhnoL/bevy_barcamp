use crate::events::{
    CapturePlayerPosition, JumpPlayer, VerifyPlayerIsAtCapturedPosition, VerifyPlayerIsInTheAir,
    WaitPlayerGrounded,
};
use crate::includes::PlayerCapturedPosition;
use crate::includes::*;
use bevy::prelude::{On, Remove};
use bevy_barcamp::game::includes::events::PlayerJump;
use bevy_barcamp::game::player::Grounded;
use bevy_barcamp::game::player::Player;
use macros::step;

const LANDING_TOLERANCE: f32 = 10.0;
const AIRBORNE_MIN_DELTA: f32 = 1.0;

pub fn provide_steps() -> Vec<Box<dyn TestStep>> {
    vec![
        step!(WaitPlayerGrounded),
        step!(CapturePlayerPosition),
        step!(JumpPlayer),
        step!(VerifyPlayerIsInTheAir),
        step!(WaitPlayerGrounded),
        step!(VerifyPlayerIsAtCapturedPosition),
    ]
}

pub fn verify_player_is_in_the_air(
    _removed: On<Remove, Grounded>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    captured_position: Res<PlayerCapturedPosition>,
    player_query: Query<&Transform, With<Player>>,
) {
    let step_name = std::any::type_name::<VerifyPlayerIsInTheAir>();
    if !unfinished_steps.0.contains(step_name) {
        return;
    }

    let  player_transform =  player_query.iter().next().expect("Player must exist");
    let baseline_position = captured_position
        .0
        .expect("Player baseline position missing before verifying airborne state");
    let current_position = player_transform.translation;

    assert!(
        (current_position.y - baseline_position.y).abs() > AIRBORNE_MIN_DELTA,
        "Expected player to have left the ground (|dy| > {AIRBORNE_MIN_DELTA}), but y went from {} to {}",
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

pub fn handle_wait_player_grounded(
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    player_grounded_query: Query<Entity, (With<Player>, With<Grounded>)>,
) {
    let step_name = std::any::type_name::<WaitPlayerGrounded>();
    if !unfinished_steps.0.contains(step_name) {
        return;
    }

    if player_grounded_query.single().is_ok() {
        unfinished_steps.remove::<WaitPlayerGrounded>();
        println!("WaitPlayerGrounded completed.");
    }
}

pub fn handle_player_jump(
    _jump_event: On<PlayerJump>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
) {    
    unfinished_steps.remove::<JumpPlayer>();
    println!("JumpPlayer completed.");
}
