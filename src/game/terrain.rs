use bevy::prelude::*;

#[derive(Default)]
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn on_init() {}

pub fn on_quit() {}
