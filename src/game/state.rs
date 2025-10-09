use bevy::prelude::States;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Uninitialized,
    Initializing,
    Running,
    Quitting,
}

impl GameState {
    /// Returns the next state in the required lifecycle order.
    pub const fn next(self) -> Self {
        match self {
            GameState::Uninitialized => GameState::Initializing,
            GameState::Initializing => GameState::Running,
            GameState::Running => GameState::Quitting,
            GameState::Quitting => GameState::Uninitialized,
        }
    }
}
