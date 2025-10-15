use bevy::prelude::*;

use crate::game::includes::events::PlayerMove;
use crate::game::includes::resources::UnfinishedStateTransitions;
use crate::game::includes::state::GameState;

const PLAYER_POSITION: Vec3 = Vec3::new(-320.0, -300.0, 0.2);
const PLAYER_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const PLAYER_Z_OFFSET: f32 = 0.1;
const PLAYER_MOVE_SPEED: f32 = 920.0;

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
        app.add_systems(OnEnter(GameState::Initializing), spawn)
            .add_systems(OnEnter(GameState::Quitting), despawn)
            .add_systems(
                Update,
                apply_player_movement.run_if(in_state(GameState::Running)),
            )
            .add_observer(on_move);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PlayerBodyPart {
    pub kind: BodyPart,
}

#[derive(Component, Debug)]
struct PlayerMovement {
    direction: f32,
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
        Name::new("Player"),
        Player,
        Transform::from_translation(PLAYER_POSITION),
        GlobalTransform::default(),
    ));

    root.with_children(|parent| {
        for (index, spec) in BODY_PART_SPECS.iter().enumerate() {
            parent.spawn((
                Name::new(format!("player-part-{}", index)),
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
    player_roots: Query<(Entity, Option<&Children>), With<Player>>,
    mut transitions: ResMut<UnfinishedStateTransitions>,
) {
    if player_roots.is_empty() {
        return;
    }

    transitions.add_one();

    for (entity, children) in player_roots.iter() {
        if let Some(children) = children {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }

        commands.entity(entity).despawn();
    }

    transitions.sub_one();
}

fn on_move(
    move_event: On<PlayerMove>,
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let (player_entity, mut transform) = player_query.iter_mut().next().expect("Player must exist");

    if move_event.active {
        let direction = f32::from(move_event.direction);
        transform.translation.x += direction * PLAYER_MOVE_SPEED * time.delta_secs();

        commands
            .entity(player_entity)
            .insert(PlayerMovement { direction });
    } else {
        commands.entity(player_entity).remove::<PlayerMovement>();
    }
}

fn apply_player_movement(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &PlayerMovement), With<Player>>,
) {
    if let Some((mut transform, movement)) = player_query.iter_mut().next() {
        transform.translation.x += movement.direction * PLAYER_MOVE_SPEED * time.delta_secs();
    };
}
