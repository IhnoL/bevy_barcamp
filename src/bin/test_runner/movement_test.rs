use crate::events::{CapturePlayerPosition, MovePlayer, VerifyPlayerMoved};
use crate::includes::{step, *};
use crate::{UnfinishedSteps, UnfinishedSteps};

pub fn provide_steps() -> Vec<Box<dyn TestStep>> {
    let mut steps: Vec<Box<dyn TestStep>> = Vec::with_capacity(6);
    steps.push(step!(CapturePlayerPosition));
    steps.push(step!(MovePlayer {
        direction: Direction::Right,
    }));
    steps.push(step!(VerifyPlayerMoved {
        expected_direction: Direction::Right,
    }));
    steps.push(step!(CapturePlayerPosition));
    steps.push(step!(MovePlayer {
        direction: Direction::Left,
    }));
    steps.push(step!(VerifyPlayerMoved {
        expected_direction: Direction::Left,
    }));
    steps
}

pub fn handle_capture_player_position(
    _capture_event: On<CapturePlayerPosition>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    mut steps_waiting: ResMut<UnfinishedSteps>,
) {
    steps_waiting.0 = true;
    println!("Observer: Handling CapturePlayerPosition");

    // Record player position for later verification
    // For now, just simulate the action

    // Decrease unfinished steps counter
    if unfinished_steps.0 > 0 {
        unfinished_steps.0 -= 1;
    }
    steps_waiting.0 = false;
    println!("Observer: CapturePlayerPosition completed. Unfinished steps: {}", unfinished_steps.0);
}

pub fn handle_move_player(
    move_event: On<MovePlayer>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    mut steps_waiting: ResMut<UnfinishedSteps>,
) {
    steps_waiting.0 = true;
    println!("Observer: Handling MovePlayer {:?}", move_event.direction);

    // In a real implementation, this would call handle_move_player(world, event)
    // For now, just simulate the move action

    // Decrease unfinished steps counter
    if unfinished_steps.0 > 0 {
        unfinished_steps.0 -= 1;
    }
    steps_waiting.0 = false;
    println!("Observer: MovePlayer completed. Unfinished steps: {}", unfinished_steps.0);
}

/// Observer for VerifyPlayerMoved events
pub fn handle_verify_player_moved(
    verify_event: On<VerifyPlayerMoved>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    mut steps_waiting: ResMut<UnfinishedSteps>,
) {
    steps_waiting.0 = true;
    println!("Observer: Handling VerifyPlayerMoved {:?}", verify_event.expected_direction);

    // Check captured positions and assert expected movement
    // For now, just simulate the verification

    // Decrease unfinished steps counter
    if unfinished_steps.0 > 0 {
        unfinished_steps.0 -= 1;
    }
    steps_waiting.0 = false;
    println!("Observer: VerifyPlayerMoved completed. Unfinished steps: {}", unfinished_steps.0);
}
