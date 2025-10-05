use bevy::prelude::*;

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Player;

pub fn on_init() {}

pub fn on_quit() {}

pub fn on_move() {}
