use bevy::prelude::*;

pub mod jump;
pub mod mob;
pub mod movement;
pub mod player;
pub mod terrain;

pub struct TestsPlugin;

impl Plugin for TestsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<movement::PlayerPositionTracker>()
            .add_observer(movement::handle_capture_player_position)
            .add_observer(movement::handle_player_move)
            .add_observer(movement::handle_verify_player_moved)
            .add_observer(terrain::handle_verify_terrain_spawned)
            .add_observer(player::handle_verify_player_spawned)
            .add_observer(mob::handle_verify_mob_spawned);
    }
}
