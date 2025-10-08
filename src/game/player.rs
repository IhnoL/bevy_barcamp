use bevy::prelude::*;

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Player;

pub fn spawn() {}

pub fn despawn() {}

pub fn on_move() {}
