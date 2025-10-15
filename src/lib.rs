pub mod game;

use crate::game::game_controller::GameControllerPlugin;
use avian2d::prelude::{Gravity, PhysicsPlugins};
use bevy::prelude::{App, DefaultPlugins, Vec2};
use bevy::state::app::AppExtStates;
use game::includes::state::GameState;

/// Configures the base Bevy app with the standard plugin stack for runtime and a minimal one during tests.
pub fn run(mut app: App) -> App {
    const GRAVITY: f32 = 2400.0;

    app.add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(Gravity(Vec2::NEG_Y * GRAVITY))
        .add_plugins(GameControllerPlugin)
        .init_state::<GameState>();
    app
}
