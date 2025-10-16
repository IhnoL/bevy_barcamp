use bevy::prelude::*;
use bevy_barcamp::game::includes::events::{Direction, PlayerJump, QuitGame, StartGame};

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
pub struct CaptureBaselineEntities;

#[derive(Clone, Event, Message, TestStep)]
pub struct CapturePlayerPosition;

#[derive(Clone, Event, TestStep)]
pub struct WaitPlayerGrounded;

#[derive(Clone, Event, Message, TestStep)]
#[step_dispatch(event = PlayerJump)]
pub struct JumpPlayer;

#[derive(Clone, Event, Message, TestStep)]
pub struct VerifyPlayerIsInTheAir;

#[derive(Clone, Event, Message, TestStep)]
pub struct VerifyPlayerIsAtCapturedPosition;

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

#[derive(Clone, Event, Message, TestStep)]
pub struct VerifyMobSpawned;

#[derive(Clone, Event, Message, TestStep)]
pub struct VerifyEntitiesDespawned;
