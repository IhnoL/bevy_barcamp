pub use bevy::prelude::*;
pub use bevy_barcamp::game::events::{Direction, PlayerMove};
use std::any::Any;

pub trait TestStep: Any + Send + Sync + 'static {
    fn send(&self, world: &mut World);

    fn as_any(&self) -> &dyn Any;
}

macro_rules! step {
    ($step:expr) => {
        Box::new($step) as Box<dyn TestStep>
    };
}

pub(crate) use step;

#[derive(Default, Resource)]
pub struct UnfinishedSteps(pub i32);
