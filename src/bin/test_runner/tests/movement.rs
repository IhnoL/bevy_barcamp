use crate::events::{CapturePlayerPosition, TriggerMovePlayer, VerifyPlayerMoved};
use crate::includes::*;
use bevy_barcamp::game::includes::events::Direction;
use macros::step;

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
    println!("Handling CapturePlayerPosition");

    unfinished_steps.sub_one();

    println!("CapturePlayerPosition completed.");
}

pub fn handle_move_player(
    move_event: On<TriggerMovePlayer>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
) {
    println!("Handling TriggerMovePlayer {:?}", move_event.direction);

    unfinished_steps.sub_one();
    println!("TriggerMovePlayer completed. ");
}

pub fn handle_verify_player_moved(
    verify_event: On<VerifyPlayerMoved>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
) {
    println!(
        "Handling VerifyPlayerMoved {:?}",
        verify_event.expected_direction
    );

    unfinished_steps.sub_one();
    println!("VerifyPlayerMoved completed.",);
}
