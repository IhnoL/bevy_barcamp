use bevy::prelude::Resource;

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
