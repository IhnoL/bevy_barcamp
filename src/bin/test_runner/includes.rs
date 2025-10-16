pub use bevy::prelude::*;
use std::any::{Any, TypeId};
use std::collections::HashSet;

pub trait TestStep: Any + Send + Sync + 'static {
    fn send(&self, world: &mut World);
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

#[derive(Default, Resource, Debug)]
pub struct PlayerCapturedPosition(pub Option<Vec3>);

#[derive(Default, Resource)]
pub struct UnfinishedSteps(pub HashSet<TypeId>);

impl UnfinishedSteps {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn add_type_id(&mut self, type_id: TypeId) {
        self.0.insert(type_id);
    }

    pub fn add<T: 'static>(&mut self) {
        self.0.insert(TypeId::of::<T>());
    }

    pub fn remove<T: 'static>(&mut self) {
        self.0.remove(&TypeId::of::<T>());
    }
}

#[derive(Default, Resource)]
pub struct PendingWaitStep(pub Option<usize>);
