use bevy::prelude::App;
use bevy_barcamp::game::includes::events::StartGame;

fn main() {
    let mut app = bevy_barcamp::init(App::new());
    app.world_mut().trigger(StartGame);
    app.run();
}
