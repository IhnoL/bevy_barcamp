pub use bevy::prelude::*;
pub use bevy_barcamp::game::events::{Direction, PlayerMove};

pub trait TestStep: Send + Sync + 'static {
    fn send(&self, world: &mut World);
}

macro_rules! step {
    ($step:expr) => {
        Box::new($step) as Box<dyn TestStep>
    };
}

pub(crate) use step;

#[derive(Default, Resource)]
pub struct UnfinishedSteps(pub i32);
