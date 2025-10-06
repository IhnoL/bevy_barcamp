use bevy::prelude::{Event, Message};

#[derive(Event)]
pub struct StartGame;

#[derive(Event)]
pub struct QuitGame;

#[derive(Event, Message)]
pub struct PlayerMove {
    pub direction: Direction,
    pub active: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}
