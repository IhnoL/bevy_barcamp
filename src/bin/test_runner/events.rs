use bevy::prelude::*;
use bevy_barcamp::game::events::{Direction, QuitGame, StartGame};

use test_step_macros::StepDispatch;

#[derive(Clone, Event, Message, StepDispatch)]
#[step_dispatch(event = StartGame)]
pub struct StartGameStep;

#[derive(Clone, Event, Message, StepDispatch)]
#[step_dispatch(event = QuitGame)]
pub struct QuitGameStep;

#[derive(Clone, Event, Message, StepDispatch)]
pub struct CapturePlayerPosition;

#[derive(Clone, Event, Message, StepDispatch)]
pub struct TriggerMovePlayer {
    pub direction: Direction,
}

#[derive(Clone, Event, Message, StepDispatch)]
pub struct VerifyPlayerMoved {
    pub expected_direction: Direction,
}

#[derive(Clone, Event, Message, StepDispatch)]
pub struct VerifyTerrainSpawned;
