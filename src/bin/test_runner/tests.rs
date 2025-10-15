use bevy::prelude::*;
use bevy_barcamp::game::includes::state::GameState;

use crate::common_handlers;
use crate::includes::PendingWaitStep;

pub mod jump;
pub mod mob;
pub mod movement;
pub mod player;
pub mod teardown;
pub mod terrain;

pub struct TestsPlugin;

impl Plugin for TestsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::includes::PlayerCapturedPosition>()
            .init_resource::<PendingWaitStep>()
            .init_resource::<teardown::BaselineEntities>()
            .add_systems(Update, common_handlers::process_wait_cycles)
            .add_systems(
                OnEnter(GameState::Running),
                common_handlers::handle_start_game,
            )
            .add_systems(
                OnEnter(GameState::Uninitialized),
                common_handlers::handle_quit_game,
            )
            .add_observer(common_handlers::handle_wait_step)
            .add_observer(common_handlers::handle_capture_player_position)
            .add_observer(jump::handle_player_jump)
            .add_observer(jump::handle_verify_player_is_in_the_air)
            .add_observer(jump::handle_verify_player_is_at_captured_position)
            .add_observer(movement::handle_player_move)
            .add_observer(movement::handle_verify_player_moved)
            .add_observer(terrain::handle_verify_terrain_spawned)
            .add_observer(player::handle_verify_player_spawned)
            .add_observer(mob::handle_verify_mob_spawned)
            .add_observer(teardown::handle_capture_baseline_entities)
            .add_observer(teardown::handle_verify_entities_despawned);
    }
}
