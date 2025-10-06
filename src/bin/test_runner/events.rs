use bevy::prelude::*;
use bevy_barcamp::game::events::Direction;

use crate::includes::TestStep;

#[derive(Event)]
pub struct StartGameStep;

#[derive(Event)]
pub struct QuitGameStep;

#[derive(Event, Message)]
pub struct CapturePlayerPosition;

#[derive(Event, Message)]
pub struct MovePlayer {
    pub direction: Direction,
}

#[derive(Event, Message)]
pub struct VerifyPlayerMoved {
    pub expected_direction: Direction,
}

impl TestStep for StartGameStep {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TestStep for QuitGameStep {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TestStep for CapturePlayerPosition {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TestStep for MovePlayer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TestStep for VerifyPlayerMoved {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
