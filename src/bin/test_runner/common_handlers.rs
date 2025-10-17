use crate::events::{CapturePlayerPosition, GenerateScreenshot, QuitGameStep, StartGameStep, WaitStep};
use crate::includes::*;
use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use std::path::Path;
pub fn handle_start_game(mut unfinished_steps: ResMut<UnfinishedSteps>) {
    unfinished_steps.remove::<StartGameStep>();
    println!("StartGameStep completed.");
}

pub fn handle_quit_game(mut unfinished_steps: ResMut<UnfinishedSteps>) {
    unfinished_steps.remove::<QuitGameStep>();
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
            unfinished_steps.remove::<WaitStep>();
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

    unfinished_steps.remove::<CapturePlayerPosition>();
    println!("CapturePlayerPosition completed.");
}

pub fn handle_generate_screenshot(
    screenshot_step: On<GenerateScreenshot>,
    mut commands: Commands,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
)
{
    let base = Path::new("src")
        .join("bin")
        .join("test_runner")
        .join("screenshots");
    let dir = if screenshot_step.is_reference{ base.join("reference") } else { base.join("last_test") };
    let path = dir.join(format!("{}.png", screenshot_step.name));
    if let Some(parent) = Path::new(&path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(path));

    unfinished_steps.remove::<GenerateScreenshot>();
    println!("GenerateScreenshot completed.");
}
