use bevy::prelude::Event;

#[derive(Event)]
pub struct StartGame;

#[derive(Event)]
pub struct QuitGame;

#[derive(Event)]
pub struct PlayerMove {
    pub direction: Direction,
    pub active: bool,
}

#[derive(Event)]
pub struct PlayerJump;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl From<Direction> for f32 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Left => -1.0,
            Direction::Right => 1.0,
        }
    }
}
