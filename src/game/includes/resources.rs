use bevy::prelude::Resource;
use derive_getters::Getters;
use crate::game::includes::state::GameState;

#[derive(Default, Resource, Getters)]
pub struct UnfinishedStateTransitions {
    #[getter(copy)]
    count: usize,
}

impl UnfinishedStateTransitions {
    pub fn add_one(&mut self) {
        self.count = self.count.saturating_add(1);
    }

    pub fn sub_one(&mut self) {
        self.count = self.count.saturating_sub(1);
    }
}

#[derive(Default, Resource)]
pub struct TargetState {
    pub state: Option<GameState>,
}
