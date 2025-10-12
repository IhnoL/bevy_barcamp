use bevy::prelude::*;

use crate::game::includes::events::PlayerMove;
use crate::game::includes::resources::UnfinishedStateTransitions;
use crate::game::includes::state::GameState;

const PLAYER_ROOT_POSITION: Vec3 = Vec3::new(-320.0, -300.0, 0.2);
const PLAYER_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const PLAYER_Z_OFFSET: f32 = 0.1;

#[derive(Clone, Copy)]
struct BodyPartSpec {
    kind: BodyPart,
    offset: Vec3,
    size: Vec2,
    rotation_radians: f32,
}

const BODY_PART_SPECS: [BodyPartSpec; 6] = [
    BodyPartSpec {
        kind: BodyPart::Head,
        offset: Vec3::new(0.0, 54.0, PLAYER_Z_OFFSET),
        size: Vec2::new(36.0, 4.0),
        rotation_radians: 0.0,
    },
    BodyPartSpec {
        kind: BodyPart::Torso,
        offset: Vec3::new(0.0, 18.0, PLAYER_Z_OFFSET),
        size: Vec2::new(4.0, 64.0),
        rotation_radians: 0.0,
    },
    BodyPartSpec {
        kind: BodyPart::ArmLeft,
        offset: Vec3::new(-26.0, 24.0, PLAYER_Z_OFFSET),
        size: Vec2::new(52.0, 4.0),
        rotation_radians: std::f32::consts::FRAC_PI_4,
    },
    BodyPartSpec {
        kind: BodyPart::ArmRight,
        offset: Vec3::new(26.0, 24.0, PLAYER_Z_OFFSET),
        size: Vec2::new(52.0, 4.0),
        rotation_radians: -std::f32::consts::FRAC_PI_4,
    },
    BodyPartSpec {
        kind: BodyPart::LegLeft,
        offset: Vec3::new(-16.0, -42.0, PLAYER_Z_OFFSET),
        size: Vec2::new(56.0, 4.0),
        rotation_radians: std::f32::consts::FRAC_PI_6,
    },
    BodyPartSpec {
        kind: BodyPart::LegRight,
        offset: Vec3::new(16.0, -42.0, PLAYER_Z_OFFSET),
        size: Vec2::new(56.0, 4.0),
        rotation_radians: -std::f32::consts::FRAC_PI_6,
    },
];

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
    ArmLeft,
    ArmRight,
    LegLeft,
    LegRight,
}

fn spawn(mut commands: Commands, mut transitions: ResMut<UnfinishedStateTransitions>) {
    transitions.add_one();

    let mut root = commands.spawn((
        Player,
        PlayerRoot,
        Transform::from_translation(PLAYER_ROOT_POSITION),
        GlobalTransform::default(),
    ));

    root.with_children(|parent| {
        for spec in BODY_PART_SPECS {
            parent.spawn((
                PlayerBodyPart { kind: spec.kind },
                Sprite::from_color(PLAYER_COLOR, spec.size),
                Transform {
                    translation: spec.offset,
                    rotation: Quat::from_rotation_z(spec.rotation_radians),
                    ..Default::default()
                },
                GlobalTransform::default(),
            ));
        }
    });

    transitions.sub_one();
}

fn despawn(
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

fn on_move(events: Option<MessageReader<PlayerMove>>, state: Res<State<GameState>>) {
    if *state.get() != GameState::Running {
        return;
    }

    if let Some(mut reader) = events {
        for _ in reader.read() {}
    }
}
