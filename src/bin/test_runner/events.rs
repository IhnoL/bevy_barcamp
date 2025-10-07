use bevy::prelude::*;
use bevy_barcamp::game::events::Direction;

use crate::includes::TestStep;

#[derive(Event, Message)]
pub struct StartGameStep;

#[derive(Event, Message)]
pub struct QuitGameStep;

#[derive(Event, Message)]
pub struct CapturePlayerPosition;

#[derive(Event, Message)]
pub struct TriggerMovePlayer {
    pub direction: Direction,
}

#[derive(Event, Message)]
pub struct VerifyPlayerMoved {
    pub expected_direction: Direction,
}

impl TestStep for StartGameStep {


    fn send(&self, world: &mut World) {
        world.write_message(self.clone());
    }
}

impl TestStep for QuitGameStep {
   
    fn send(&self, world: &mut World) {
        world.write_message(self.clone());
    }
}

impl TestStep for CapturePlayerPosition {


    fn send(&self, world: &mut World) {
        world.write_message(self.clone());
    }
}

impl TestStep for TriggerMovePlayer {
 

    fn send(&self, world: &mut World) {
        world.write_message(self.clone());
    }
}

impl TestStep for VerifyPlayerMoved {


    fn send(&self, world: &mut World) {
        world.write_message(self.clone());
    }
}
