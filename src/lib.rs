pub mod game;

use crate::game::game_controller::GameControllerPlugin;
use avian2d::prelude::{Gravity, PhysicsPlugins};
use bevy::app::PluginGroup;
use bevy::prelude::{default, App, DefaultPlugins, Vec2, Window, WindowPlugin};
use bevy::state::app::AppExtStates;
use bevy::window::PresentMode;
use game::includes::state::GameState;

/// Configures the base Bevy app with the standard plugin stack for runtime and a minimal one during tests.
pub fn init(mut app: App) -> App {
    const GRAVITY: f32 = 2400.0;

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }),
        PhysicsPlugins::default(),
    ))
    .insert_resource(Gravity(Vec2::NEG_Y * GRAVITY))
    .add_plugins(GameControllerPlugin)
    .init_state::<GameState>();
    app
}
