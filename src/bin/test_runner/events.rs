use bevy::prelude::*;
use bevy_barcamp::game::includes::events::{Direction, QuitGame, StartGame};

use macros::TestStep;

#[derive(Clone, Event, Message, TestStep)]
#[step_dispatch(event = StartGame)]
pub struct StartGameStep;

#[derive(Clone, Event, Message, TestStep)]
#[step_dispatch(event = QuitGame)]
pub struct QuitGameStep;

#[derive(Clone, Event, TestStep)]
pub struct WaitStep {
    pub updates: usize,
}

#[derive(Clone, Event, Message, TestStep)]
pub struct CapturePlayerPosition;

#[derive(Clone, Event, Message, TestStep)]
pub struct TriggerPlayerMove {
    pub direction: Direction,
}

#[derive(Clone, Event, Message, TestStep)]
pub struct VerifyPlayerMoved {
    pub expected_direction: Direction,
}

#[derive(Clone, Event, Message, TestStep)]
pub struct VerifyTerrainSpawned;

#[derive(Clone, Event, Message, TestStep)]
pub struct VerifyPlayerSpawned;
