use bevy::prelude::*;

use super::terrain::TerrainPlugin;

#[derive(Default)]
pub struct GameControllerPlugin;

impl Plugin for GameControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TerrainPlugin);
    }
}

pub fn on_start_game() {}

pub fn on_quit_game() {}
