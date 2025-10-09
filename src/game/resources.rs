use bevy::prelude::Resource;

use super::state::GameState;

#[derive(Default, Resource)]
pub struct UnfinishedStateTransitions {
     count: usize,
}

impl UnfinishedStateTransitions {
    pub fn add_one(&mut self) {
        self.count = self.count.saturating_add(1);
    }

    pub fn sub_one(&mut self) {
        self.count = self.count.saturating_sub(1);
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

#[derive(Default, Resource)]
pub struct TargetState {
    target: Option<GameState>,
}

impl TargetState {
    pub fn set(&mut self, state: GameState) {
        self.target = Some(state);
    }

    pub fn clear(&mut self) {
        self.target = None;
    }

    pub fn get(&self) -> Option<GameState> {
        self.target
    }

    pub fn is_set(&self) -> bool {
        self.target.is_some()
    }
}
