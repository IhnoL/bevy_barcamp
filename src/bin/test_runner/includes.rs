pub use bevy::prelude::*;
pub use bevy_barcamp::game::events::{Direction, PlayerMove};

pub use crate::{QuitGameStep, StartGameStep, StepsWaiting, TestQueue};

#[derive(Default)]
pub struct TestEvents {
    pub steps: Vec<Box<dyn TestStep>>,
}

pub trait TestStep: Send + Sync + 'static {}

#[derive(Default, Resource)]
pub struct StepsWaiting;