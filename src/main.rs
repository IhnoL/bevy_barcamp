use bevy::prelude::App;

use bevy_barcamp::game::events::StartGame;

fn main() {
    let mut app = bevy_barcamp::run(App::new());
    app.world_mut().trigger(StartGame);
    app.run();
}
