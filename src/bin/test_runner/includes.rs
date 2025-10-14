pub use bevy::prelude::*;

pub trait TestStep: Send + Sync + 'static {
    fn send(&self, world: &mut World);
}

#[derive(Default, Resource, Debug)]
pub struct PlayerCapturedPosition(pub Option<Vec3>);

#[derive(Default, Resource)]
pub struct UnfinishedSteps(pub usize);

impl UnfinishedSteps {
    pub fn sub_one(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }

    pub fn add_one(&mut self) {
        self.0 = self.0.saturating_add(1);
    }
}

#[derive(Default, Resource)]
pub struct PendingWaitStep(pub Option<usize>);

