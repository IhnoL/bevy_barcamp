use crate::events::{CapturePlayerPosition, TriggerPlayerMove, VerifyPlayerMoved, WaitStep};
use crate::includes::*;
use bevy_barcamp::game::includes::events::{Direction, PlayerMove};
use bevy_barcamp::game::player::Player;
use macros::step;

const MIN_MOVEMENT_DELTA: f32 = 20.0;

pub fn provide_steps() -> Vec<Box<dyn TestStep>> {
    vec![
        step!(CapturePlayerPosition),
        step!(TriggerPlayerMove {
            direction: Direction::Right,
        }),
        step!(WaitStep { updates: 30 }),
        step!(VerifyPlayerMoved {
            expected_direction: Direction::Right,
        }),
        step!(CapturePlayerPosition),
        step!(TriggerPlayerMove {
            direction: Direction::Left,
        }),
        step!(WaitStep { updates: 30 }),
        step!(VerifyPlayerMoved {
            expected_direction: Direction::Left,
        }),
    ]
}

pub fn handle_player_move(
    move_event: On<TriggerPlayerMove>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    mut commands: Commands,
) {
    commands.trigger(PlayerMove {
        direction: move_event.direction,
        active: true,
    });

    unfinished_steps.remove::<TriggerPlayerMove>();
    println!("TriggerMovePlayer completed.");
}

pub fn handle_verify_player_moved(
    verify_event: On<VerifyPlayerMoved>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    mut captured_position: ResMut<PlayerCapturedPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
) {
    let previous_position = captured_position
        .0
        .unwrap_or_else(|| panic!("Player position was not captured before verification"));

    let mut player_iter = player_query.iter();
    let current_transform = player_iter
        .next()
        .expect("Player root entity not found during verification");
    let current_position = current_transform.translation;

    match verify_event.expected_direction {
        Direction::Right => assert!(
            current_position.x > previous_position.x + MIN_MOVEMENT_DELTA,
            "Expected player to move right by at least {MIN_MOVEMENT_DELTA}, but x went from {} to {}",
            previous_position.x,
            current_position.x
        ),
        Direction::Left => assert!(
            current_position.x < previous_position.x - MIN_MOVEMENT_DELTA,
            "Expected player to move left by at least {MIN_MOVEMENT_DELTA}, but x went from {} to {}",
            previous_position.x,
            current_position.x
        ),
    }

    captured_position.0 = Some(current_position);

    commands.trigger(PlayerMove {
        direction: verify_event.expected_direction,
        active: false,
    });

    unfinished_steps.remove::<VerifyPlayerMoved>();
    println!("VerifyPlayerMoved completed.");
}
