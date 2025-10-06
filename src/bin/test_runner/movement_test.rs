use crate::events::{CapturePlayerPosition, MovePlayer, VerifyPlayerMoved};
use crate::includes::{step, *};

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

#[allow(dead_code)]
pub fn handle_capture_player_position(mut _events: EventReader<CapturePlayerPosition>) {
    todo!("Record player position for later verification");
}

#[allow(dead_code)]
pub fn handle_move_player(world: &mut World, step: &MovePlayer) {
    world.write_message(PlayerMove {
        direction: step.direction,
        active: true,
    });
}

#[allow(dead_code)]
pub fn handle_verify_player_moved(mut _events: EventReader<VerifyPlayerMoved>) {
    todo!("Check captured positions and assert expected movement");
}
