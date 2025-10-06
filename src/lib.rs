pub mod game;

use bevy::prelude::{App, DefaultPlugins};

pub fn run(mut app: App) {
    app.add_plugins(DefaultPlugins);
    app.run();
}
