use crate::test_types::{MoveDirection, MoveEvent};

pub struct MovementStepHandler;

impl MovementStepHandler {
    pub fn capture_player_position(&mut self) {
        todo!("Capture player position for movement test");
    }

    pub fn send_move_event(&mut self, _event: MoveEvent) {
        todo!("Dispatch move event for movement test");
    }

    pub fn verify_player_moved(&self, _direction: MoveDirection) {
        todo!("Verify player movement for the specified direction");
    }
}
