pub use bevy::prelude::*;
pub use bevy_barcamp::game::events::{Direction, PlayerMove};

#[allow(unused_imports)]
pub use crate::TestQueue;

pub trait TestStep: Send + Sync + 'static {
    fn as_any(&self) -> &dyn std::any::Any;
    fn send(&self, world: &mut World);
}

macro_rules! step {
    ($step:expr) => {
        Box::new($step) as Box<dyn TestStep>
    };
}

pub(crate) use step;

/// Resource to track unfinished test steps
#[derive(Default, Resource)]
pub struct UnfinishedSteps(pub u32);

/// Resource to track if steps are currently waiting
#[derive(Default, Resource)]
pub struct UnfinishedSteps(pub bool);