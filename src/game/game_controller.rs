use bevy::prelude::*;

use super::{camera::CameraPlugin, terrain::TerrainPlugin};

#[derive(Default)]
pub struct GameControllerPlugin;

impl Plugin for GameControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TerrainPlugin, CameraPlugin));
    }
}

pub fn on_start_game() {}

pub fn on_quit_game() {}
