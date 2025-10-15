use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

use crate::game::includes::events::{Direction, PlayerJump, PlayerMove};
use crate::game::includes::state::GameState;

#[derive(Default)]
pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_keyboard_input.run_if(in_state(GameState::Running)),
        );
    }
}

fn handle_keyboard_input(keyboard: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    for (key, direction) in [
        (KeyCode::KeyA, Direction::Left),
        (KeyCode::KeyD, Direction::Right),
    ] {
        if keyboard.just_pressed(key) {
            commands.trigger(PlayerMove {
                direction,
                active: true,
            });
        }

        if keyboard.just_released(key) {
            commands.trigger(PlayerMove {
                direction,
                active: false,
            });
        }
    }

    if keyboard.just_pressed(KeyCode::Space) {
        commands.trigger(PlayerJump);
    }
}
