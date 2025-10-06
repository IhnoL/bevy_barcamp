use crate::includes::*;

pub fn provide_steps() -> TestEvents {
    todo!("Return movement test steps using Capture/Move/Verify events");
}

#[derive(Event)]
pub struct CapturePlayerPosition;

#[derive(Event)]
pub struct MovePlayer {
    pub direction: Direction,
}

#[derive(Event)]
pub struct VerifyPlayerMoved {
    pub expected_direction: Direction,
}

pub fn handle_capture_player_position(mut _events: EventReader<CapturePlayerPosition>) {
    todo!("Record player position for later verification");
}

pub fn handle_move_player(
    mut _player_moves: EventWriter<PlayerMove>,
) {
    todo!("Convert queued MovePlayer step into the real PlayerMove event");
}

pub fn handle_verify_player_moved(mut _events: EventReader<VerifyPlayerMoved>) {
    todo!("Check captured positions and assert expected movement");
}

impl TestStep for CapturePlayerPosition {}

impl TestStep for MovePlayer {}

impl TestStep for VerifyPlayerMoved {}
