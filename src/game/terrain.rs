use crate::game::includes::resources::UnfinishedStateTransitions;
use crate::game::includes::state::GameState;
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
const GROUND_SIZE: Vec2 = Vec2::new(2500.0, 20.0);
const GROUND_Y: f32 = -360.0;
const GROUND_X: f32 = -500.0;
const PLATFORM_SIZE: Vec2 = Vec2::new(140.0, 20.0);
const PLATFORM_POSITIONS: [Vec3; 3] = [
    Vec3::new(-200.0, 80.0, 0.1),
    Vec3::new(0.0, -150.0, 0.1),
    Vec3::new(200.0, 40.0, 0.1),
];

#[derive(Default)]
pub struct TerrainPlugin;

#[derive(Component)]
pub struct TerrainRoot;

#[derive(Component)]
pub struct TerrainPiece;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), spawn)
            .add_systems(OnEnter(GameState::Quitting), despawn);
    }
}

pub fn spawn(
    mut commands: Commands,
    existing_terrain_roots: Query<(), With<TerrainRoot>>,
    mut transitions: ResMut<UnfinishedStateTransitions>,
) {
    if !existing_terrain_roots.is_empty() {
        return;
    }

    transitions.add_one();

    let mut root = commands.spawn((
        Name::new("Terrain"),
        TerrainRoot,
        Transform::default(),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::default(),
    ));

    root.with_children(|parent| {
        parent.spawn((
            Name::new("Ground"),
            TerrainPiece,
            RigidBody::Static,
            Collider::rectangle(GROUND_SIZE.x, GROUND_SIZE.y),
            Sprite::from_color(Color::srgb(0.4, 0.3, 0.2), GROUND_SIZE),
            Transform::from_xyz(GROUND_X, GROUND_Y, 0.0),
            GlobalTransform::default(),
        ));

        for (index, translation) in PLATFORM_POSITIONS.iter().enumerate() {
            parent.spawn((
                Name::new(format!("Platform{}", index + 1)),
                TerrainPiece,
                RigidBody::Static,
                Collider::rectangle(PLATFORM_SIZE.x, PLATFORM_SIZE.y),
                Sprite::from_color(Color::srgb(0.6, 0.6, 0.6), PLATFORM_SIZE),
                Transform::from_translation(*translation),
                GlobalTransform::default(),
            ));
        }
    });

    transitions.sub_one();
}

pub fn despawn(
    mut commands: Commands,
    terrain_root_query: Query<Entity, With<TerrainRoot>>,
    terrain_piece_query: Query<Entity, With<TerrainPiece>>,
    mut transitions: ResMut<UnfinishedStateTransitions>,
) {
    if terrain_piece_query.is_empty() && terrain_root_query.is_empty() {
        return;
    }

    transitions.add_one();

    for entity in terrain_piece_query.iter() {
        commands.entity(entity).despawn();
    }

    for entity in terrain_root_query.iter() {
        commands.entity(entity).despawn();
    }

    transitions.sub_one();
}
