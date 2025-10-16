pub use bevy::prelude::*;
use std::any::Any;
use std::collections::HashSet;

pub trait TestStep: Any + Send + Sync + 'static + std::fmt::Display {
    fn send(&self, world: &mut World);
}

#[derive(Default, Resource, Debug)]
pub struct PlayerCapturedPosition(pub Option<Vec3>);

#[derive(Default, Resource)]
pub struct UnfinishedSteps(pub HashSet<String>);

impl UnfinishedSteps {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn add<S: Into<String>>(&mut self, name: S) {
        self.0.insert(name.into());
    }

    pub fn remove<T: 'static>(&mut self) {
        let name: &str = std::any::type_name::<T>();
        self.0.remove(name);
    }
}

#[derive(Default, Resource)]
pub struct PendingWaitStep(pub Option<usize>);
