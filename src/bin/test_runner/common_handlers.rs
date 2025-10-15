use crate::events::{CapturePlayerPosition, WaitStep};
use crate::includes::*;
use bevy::prelude::*;
pub fn handle_start_game(mut unfinished_steps: ResMut<UnfinishedSteps>) {
    unfinished_steps.sub_one();
    println!("StartGameStep completed.");
}

pub fn handle_quit_game(mut unfinished_steps: ResMut<UnfinishedSteps>) {
    unfinished_steps.sub_one();
    println!("QuitGameStep completed.");
}

pub fn handle_wait_step(wait_step: On<WaitStep>, mut pending: ResMut<PendingWaitStep>) {
    println!("Handling WaitStep waiting");
    pending.0 = Some(wait_step.updates);
}

pub fn process_wait_cycles(
    mut pending: ResMut<PendingWaitStep>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
) {
    if let Some(wait_cycles) = pending.0.as_mut() {
        println!("Still waiting..");
        *wait_cycles = wait_cycles.saturating_sub(1);
        if *wait_cycles == 0 {
            println!("WaitStep completed.");
            pending.0 = None;
            unfinished_steps.sub_one();
        }
    }
}

pub fn handle_capture_player_position(
    _capture_event: On<CapturePlayerPosition>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    player_query: Query<&Transform, With<bevy_barcamp::game::player::Player>>,
    mut captured_position: ResMut<PlayerCapturedPosition>,
) {
    let mut player_iter = player_query.iter();
    let transform = player_iter
        .next()
        .expect("Player root entity not found when capturing position");
    captured_position.0 = Some(transform.translation);

    unfinished_steps.sub_one();
    println!("CapturePlayerPosition completed.");
}