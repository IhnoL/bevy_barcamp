use bevy::prelude::*;

use crate::game::includes::events::PlayerMove;
use crate::game::includes::resources::UnfinishedStateTransitions;
use crate::game::includes::state::GameState;

const PLAYER_ROOT_POSITION: Vec3 = Vec3::new(-320.0, -300.0, 0.2);
const PLAYER_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const HEAD_SIZE: Vec2 = Vec2::new(42.0, 4.0);
const TORSO_SIZE: Vec2 = Vec2::new(4.0, 64.0);
const ARMS_SIZE: Vec2 = Vec2::new(70.0, 4.0);
const LEGS_SIZE: Vec2 = Vec2::new(4.0, 60.0);
const HEAD_OFFSET_Y: f32 = 54.0;
const ARMS_OFFSET_Y: f32 = 20.0;
const LEGS_OFFSET_Y: f32 = -40.0;

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Running), spawn)
            .add_systems(OnEnter(GameState::Quitting), despawn)
            .add_systems(Update, on_move);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerRoot;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PlayerBodyPart {
    pub kind: BodyPart,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BodyPart {
    Head,
    Torso,
    Arms,
    Legs,
}

impl BodyPart {
    pub const fn label(self) -> &'static str {
        match self {
            BodyPart::Head => "Head",
            BodyPart::Torso => "Torso",
            BodyPart::Arms => "Arms",
            BodyPart::Legs => "Legs",
        }
    }
}

pub fn spawn(
    mut commands: Commands,
    existing: Query<(), With<Player>>,
    mut transitions: ResMut<UnfinishedStateTransitions>,
) {
    if !existing.is_empty() {
        return;
    }

    transitions.add_one();

    let mut root = commands.spawn((
        Name::new("Player"),
        Player,
        PlayerRoot,
        Transform::from_translation(PLAYER_ROOT_POSITION),
        GlobalTransform::default(),
    ));

    root.with_children(|parent| {
        parent.spawn((
            Name::new(format!("Player{}", BodyPart::Head.label())),
            PlayerBodyPart {
                kind: BodyPart::Head,
            },
            Sprite::from_color(PLAYER_COLOR, HEAD_SIZE),
            Transform {
                translation: Vec3::new(0.0, HEAD_OFFSET_Y, 0.1),
                ..Default::default()
            },
            GlobalTransform::default(),
        ));

        parent.spawn((
            Name::new(format!("Player{}", BodyPart::Torso.label())),
            PlayerBodyPart {
                kind: BodyPart::Torso,
            },
            Sprite::from_color(PLAYER_COLOR, TORSO_SIZE),
            Transform {
                translation: Vec3::new(0.0, 0.0, 0.1),
                ..Default::default()
            },
            GlobalTransform::default(),
        ));

        parent.spawn((
            Name::new(format!("Player{}", BodyPart::Arms.label())),
            PlayerBodyPart {
                kind: BodyPart::Arms,
            },
            Sprite::from_color(PLAYER_COLOR, ARMS_SIZE),
            Transform {
                translation: Vec3::new(0.0, ARMS_OFFSET_Y, 0.1),
                ..Default::default()
            },
            GlobalTransform::default(),
        ));

        parent.spawn((
            Name::new(format!("Player{}", BodyPart::Legs.label())),
            PlayerBodyPart {
                kind: BodyPart::Legs,
            },
            Sprite::from_color(PLAYER_COLOR, LEGS_SIZE),
            Transform {
                translation: Vec3::new(0.0, LEGS_OFFSET_Y, 0.1),
                rotation: Quat::from_rotation_z(0.25 * std::f32::consts::PI),
                ..Default::default()
            },
            GlobalTransform::default(),
        ));
    });

    transitions.sub_one();
}

pub fn despawn(
    mut commands: Commands,
    roots: Query<Entity, With<PlayerRoot>>,
    parts: Query<Entity, With<PlayerBodyPart>>,
    mut transitions: ResMut<UnfinishedStateTransitions>,
) {
    if roots.is_empty() && parts.is_empty() {
        return;
    }

    transitions.add_one();

    for entity in parts.iter() {
        commands.entity(entity).despawn();
    }

    for entity in roots.iter() {
        commands.entity(entity).despawn();
    }

    transitions.sub_one();
}

pub fn on_move(events: Option<MessageReader<PlayerMove>>, state: Res<State<GameState>>) {
    if *state.get() != GameState::Running {
        return;
    }

    if let Some(mut reader) = events {
        for _ in reader.read() {}
    }
}
