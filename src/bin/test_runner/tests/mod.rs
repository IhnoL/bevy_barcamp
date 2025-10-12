use bevy::prelude::*;

pub mod jump;
pub mod movement;
pub mod player;
pub mod terrain;

pub struct TestsPlugin;

impl Plugin for TestsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(movement::handle_capture_player_position)
            .add_observer(movement::handle_move_player)
            .add_observer(movement::handle_verify_player_moved)
            .add_observer(terrain::handle_verify_terrain_spawned)
            .add_observer(player::handle_verify_player_spawned);
    }
}
