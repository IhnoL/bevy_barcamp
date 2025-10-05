use bevy::prelude::*;

#[derive(Default)]
pub struct GameControllerPlugin;

impl Plugin for GameControllerPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn on_start_game() {}

pub fn on_quit_game() {}
