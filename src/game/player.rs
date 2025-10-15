use avian2d::prelude::{Collider, CollidingEntities, LinearVelocity, LockedAxes, RigidBody};
use bevy::prelude::*;

use crate::game::includes::events::{Direction, PlayerJump, PlayerMove};
use crate::game::includes::resources::UnfinishedStateTransitions;
use crate::game::includes::state::GameState;
use crate::game::terrain::TerrainPiece;

const PLAYER_POSITION: Vec3 = Vec3::new(-320.0, -200.0, 0.2);
const PLAYER_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const PLAYER_Z_OFFSET: f32 = 0.1;
const PLAYER_MOVE_SPEED: f32 = 920.0;
const PLAYER_COLLIDER_SIZE: Vec2 = Vec2::new(48.0, 120.0);
const PLAYER_JUMP_SPEED: f32 = 1200.0;

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
        offset: Vec3::new(0.0, 34.0, PLAYER_Z_OFFSET),
        size: Vec2::new(36.0, 4.0),
        rotation_radians: 0.0,
    },
    BodyPartSpec {
        kind: BodyPart::Torso,
        offset: Vec3::new(0.0, -2.0, PLAYER_Z_OFFSET),
        size: Vec2::new(4.0, 64.0),
        rotation_radians: 0.0,
    },
    BodyPartSpec {
        kind: BodyPart::ArmLeft,
        offset: Vec3::new(-26.0, 4.0, PLAYER_Z_OFFSET),
        size: Vec2::new(52.0, 4.0),
        rotation_radians: std::f32::consts::FRAC_PI_4,
    },
    BodyPartSpec {
        kind: BodyPart::ArmRight,
        offset: Vec3::new(26.0, 4.0, PLAYER_Z_OFFSET),
        size: Vec2::new(52.0, 4.0),
        rotation_radians: -std::f32::consts::FRAC_PI_4,
    },
    BodyPartSpec {
        kind: BodyPart::LegLeft,
        offset: Vec3::new(-16.0, -62.0, PLAYER_Z_OFFSET),
        size: Vec2::new(56.0, 4.0),
        rotation_radians: std::f32::consts::FRAC_PI_6,
    },
    BodyPartSpec {
        kind: BodyPart::LegRight,
        offset: Vec3::new(16.0, -62.0, PLAYER_Z_OFFSET),
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
            .add_observer(on_move)
            .add_observer(on_jump);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PlayerBodyPart {
    pub kind: BodyPart,
}

#[derive(Component, Clone, Copy, Debug)]
struct PlayerMovement {
    direction: Direction,
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
        RigidBody::Dynamic,
        Collider::rectangle(PLAYER_COLLIDER_SIZE.x, PLAYER_COLLIDER_SIZE.y),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        CollidingEntities::default(),
        Transform::from_translation(PLAYER_POSITION),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::default(),
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
) {
    let (player_entity, mut transform) =
        player_query.iter_mut().next().expect("Player must exist");

    if move_event.active {
        let direction = f32::from(move_event.direction);
        transform.translation.x += direction * PLAYER_MOVE_SPEED * 0.03;

        commands
            .entity(player_entity)
            .insert(PlayerMovement {
                direction: move_event.direction,
            });
    } else {
        commands.entity(player_entity).remove::<PlayerMovement>();
    }
}

fn apply_player_movement(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &PlayerMovement), With<Player>>,
) {
    if let Some((mut transform, movement)) = player_query.iter_mut().next() {
        let displacement = f32::from(movement.direction) * PLAYER_MOVE_SPEED * time.delta_secs();
        transform.translation.x += displacement;
    }
}

fn on_jump(
    _jump_event: On<PlayerJump>,
    mut player_query: Query<(&CollidingEntities, &mut LinearVelocity), With<Player>>,
    terrain_query: Query<(), With<TerrainPiece>>,
) {
    let (collisions, mut velocity) = player_query.iter_mut().next() .expect("Player must exist"); 
    if player_is_grounded(collisions, &terrain_query) {
        velocity.y = PLAYER_JUMP_SPEED.max(velocity.y);
    }
}

fn player_is_grounded(
    collisions: &CollidingEntities,
    terrain_query: &Query<(), With<TerrainPiece>>,
) -> bool {
    collisions
        .iter()
        .any(|entity| terrain_query.get(*entity).is_ok())
}
