use crate::events::{CapturePlayerPosition, TriggerPlayerMove, VerifyPlayerMoved};
use crate::includes::*;
use bevy_barcamp::game::includes::events::{Direction, PlayerMove};
use bevy_barcamp::game::includes::state::GameState;
use bevy_barcamp::game::player::PlayerRoot;
use macros::step;

const MIN_MOVEMENT_DELTA: f32 = 0.25;

#[derive(Default, Resource, Debug)]
pub struct PlayerPositionTracker {
    last_position: Option<Vec3>,
}

pub fn provide_steps() -> Vec<Box<dyn TestStep>> {
    vec![
        step!(CapturePlayerPosition),
        step!(TriggerPlayerMove {
            direction: Direction::Right,
        }),
        step!(VerifyPlayerMoved {
            expected_direction: Direction::Right,
        }),
        step!(CapturePlayerPosition),
        step!(TriggerPlayerMove {
            direction: Direction::Left,
        }),
        step!(VerifyPlayerMoved {
            expected_direction: Direction::Left,
        }),
    ]
}

pub fn handle_capture_player_position(
    _capture_event: On<CapturePlayerPosition>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    state: Res<State<GameState>>,
    root_query: Query<&Transform, With<PlayerRoot>>,
    mut tracker: ResMut<PlayerPositionTracker>,
) {
    println!("Handling CapturePlayerPosition");

    if *state.get() != GameState::Running {
        panic!("CapturePlayerPosition triggered outside of GameState::Running");
    }

    let mut roots = root_query.iter();
    let transform = roots
        .next()
        .unwrap_or_else(|| panic!("Player root entity not found when capturing position"));
    tracker.last_position = Some(transform.translation);

    unfinished_steps.sub_one();
    println!("CapturePlayerPosition completed.");
}

pub fn handle_player_move(
    move_event: On<TriggerPlayerMove>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    mut commands: Commands,
    state: Res<State<GameState>>,
) {
    println!("Handling TriggerMovePlayer {:?}", move_event.direction);

    if *state.get() != GameState::Running {
        panic!("TriggerMovePlayer fired outside of GameState::Running");
    }

    commands.trigger(PlayerMove {
        direction: move_event.direction,
        active: true,
    });

    unfinished_steps.sub_one();
    println!("TriggerMovePlayer completed.");
}

pub fn handle_verify_player_moved(
    verify_event: On<VerifyPlayerMoved>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    mut tracker: ResMut<PlayerPositionTracker>,
    root_query: Query<&Transform, With<PlayerRoot>>,
    mut commands: Commands,
    state: Res<State<GameState>>,
) {
    println!(
        "Handling VerifyPlayerMoved {:?}",
        verify_event.expected_direction
    );

    if *state.get() != GameState::Running {
        panic!("VerifyPlayerMoved fired outside of GameState::Running");
    }

    let previous_position = tracker
        .last_position
        .unwrap_or_else(|| panic!("Player position was not captured before verification"));

    let mut roots = root_query.iter();
    let current_transform = roots
        .next()
        .unwrap_or_else(|| panic!("Player root entity not found during verification"));
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

    tracker.last_position = Some(current_position);

    commands.trigger(PlayerMove {
        direction: verify_event.expected_direction,
        active: false,
    });

    unfinished_steps.sub_one();
    println!("VerifyPlayerMoved completed.");
}
