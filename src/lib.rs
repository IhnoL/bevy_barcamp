pub mod game;

use bevy::prelude::{App, DefaultPlugins};

pub fn run() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.run();
}
