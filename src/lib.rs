pub mod game;

use bevy::prelude::{App, DefaultPlugins};
use bevy::state::app::AppExtStates;

use crate::game::{game_controller::GameControllerPlugin, state::GameState};

/// Configures the base Bevy app with the standard plugin stack for runtime and a minimal one during tests.
pub fn run(mut app: App) -> App {
    app.add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .init_state::<GameState>();
    app
}
