use bevy::prelude::*;
use bevy_barcamp::game::includes::state::GameState;
use crate::common_handlers;
use crate::includes::PendingWaitStep;

pub mod jump_test;
pub mod mob_test;
pub mod movement_test;
pub mod player_test;
pub mod teardown_test;
pub mod terrain_test;

pub struct TestsPlugin;

impl Plugin for TestsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::includes::PlayerCapturedPosition>()
            .init_resource::<PendingWaitStep>()
            .init_resource::<teardown_test::BaselineEntities>()
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
            .add_observer(common_handlers::handle_generate_screenshot)
            .add_observer(jump_test::handle_player_jump)
            .add_observer(jump_test::verify_player_is_in_the_air)
            .add_observer(jump_test::handle_verify_player_is_at_captured_position)
            .add_systems(Update, jump_test::handle_wait_player_grounded)
            .add_observer(movement_test::handle_player_move)
            .add_observer(movement_test::handle_verify_player_moved)
            .add_observer(terrain_test::handle_verify_terrain_spawned)
            .add_observer(player_test::handle_verify_player_spawned)
            .add_observer(mob_test::handle_verify_mob_spawned)
            .add_observer(teardown_test::handle_capture_baseline_entities)
            .add_observer(teardown_test::handle_verify_entities_despawned);
    }
}
