use bevy::prelude::States;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Uninitialized,
    Initializing,
    Running,
    Quitting,
}
