use crate::events::{CapturePlayerPosition, TriggerMovePlayer, VerifyPlayerMoved};
use crate::includes::{step, *};

pub fn provide_steps() -> Vec<Box<dyn TestStep>> {
    let mut steps: Vec<Box<dyn TestStep>> = Vec::with_capacity(6);
    steps.push(step!(CapturePlayerPosition));
    steps.push(step!(TriggerMovePlayer {
        direction: Direction::Right,
    }));
    steps.push(step!(VerifyPlayerMoved {
        expected_direction: Direction::Right,
    }));
    steps.push(step!(CapturePlayerPosition));
    steps.push(step!(TriggerMovePlayer {
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
) {
    unfinished_steps.0 += 1;
    println!("Observer: Handling CapturePlayerPosition");

    unfinished_steps.0 -= 1;

    println!(
        "Observer: CapturePlayerPosition completed. Unfinished steps: {}",
        unfinished_steps.0
    );
}

pub fn handle_move_player(
    move_event: On<TriggerMovePlayer>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
) {
    unfinished_steps.0 += 1;
    println!("Observer: Handling TriggerMovePlayer {:?}", move_event.direction);

    unfinished_steps.0 -= 1;
    println!(
        "Observer: TriggerMovePlayer completed. Unfinished steps: {}",
        unfinished_steps.0
    );
}

/// Observer for VerifyPlayerMoved events
pub fn handle_verify_player_moved(
    verify_event: On<VerifyPlayerMoved>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
) {
    unfinished_steps.0 += 1;
    println!("Observer: Handling VerifyPlayerMoved {:?}", verify_event.expected_direction);

    unfinished_steps.0 -= 1;
    println!(
        "Observer: VerifyPlayerMoved completed. Unfinished steps: {}",
        unfinished_steps.0
    );
}
