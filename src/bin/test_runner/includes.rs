pub use bevy::prelude::*;
pub use bevy_barcamp::game::events::{Direction, PlayerMove};

#[allow(unused_imports)]
pub use crate::TestQueue;

pub trait TestStep: Send + Sync + 'static {
    fn as_any(&self) -> &dyn std::any::Any;
}

macro_rules! step {
    ($step:expr) => {
        Box::new($step) as Box<dyn TestStep>
    };
}

pub(crate) use step;

#[derive(Default, Resource)]
#[allow(dead_code)]
pub struct StepsWaiting;
