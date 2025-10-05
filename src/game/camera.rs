use bevy::prelude::*;


#[derive(Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn on_init() {}

pub fn on_quit() {}
