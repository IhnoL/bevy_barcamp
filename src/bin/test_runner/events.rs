use bevy::prelude::*;
use bevy_barcamp::game::events::Direction;

use crate::includes::TestStep;

#[derive(Clone, Event, Message)]
pub struct StartGameStep;

#[derive(Clone, Event, Message)]
pub struct QuitGameStep;

#[derive(Clone, Event, Message)]
pub struct CapturePlayerPosition;

#[derive(Clone, Event, Message)]
pub struct TriggerMovePlayer {
    pub direction: Direction,
}

#[derive(Clone, Event, Message)]
pub struct VerifyPlayerMoved {
    pub expected_direction: Direction,
}

impl TestStep for StartGameStep {


    fn send(&self, world: &mut World) {
        world.trigger(self.clone());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TestStep for QuitGameStep {
   
    fn send(&self, world: &mut World) {
        world.trigger(self.clone());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TestStep for CapturePlayerPosition {


    fn send(&self, world: &mut World) {
        world.trigger(self.clone());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TestStep for TriggerMovePlayer {
 

    fn send(&self, world: &mut World) {
        world.trigger(self.clone());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TestStep for VerifyPlayerMoved {


    fn send(&self, world: &mut World) {
        world.trigger(self.clone());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
