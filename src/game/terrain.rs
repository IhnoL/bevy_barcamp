use bevy::prelude::*;

use super::{
    resources::UnfinishedStateTransitions,
    state::GameState,
};

const GROUND_SIZE: Vec2 = Vec2::new(800.0, 40.0);
const GROUND_Y: f32 = -360.0;
const PLATFORM_SIZE: Vec2 = Vec2::new(140.0, 20.0);
const PLATFORM_POSITIONS: [Vec3; 5] = [
    Vec3::new(-260.0, -120.0, 0.1),
    Vec3::new(-80.0, -20.0, 0.1),
    Vec3::new(120.0, -60.0, 0.1),
    Vec3::new(200.0, 40.0, 0.1),
    Vec3::new(-40.0, 120.0, 0.1),
];

#[derive(Default)]
pub struct TerrainPlugin;

#[derive(Component)]
pub struct TerrainRoot;

#[derive(Component)]
pub struct TerrainPiece;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Running), spawn)
            .add_systems(OnExit(GameState::Running), despawn);
    }
}

pub fn spawn(
    mut commands: Commands,
    existing: Query<(), With<TerrainRoot>>,
    mut transitions: ResMut<UnfinishedStateTransitions>,
) {
    if !existing.is_empty() {
        return;
    }

    transitions.add_one();

    let mut root = commands.spawn((
        Name::new("Terrain"),
        TerrainRoot,
        Transform::default(),
        GlobalTransform::default(),
    ));

    root.with_children(|parent| {
        parent.spawn((
            Name::new("Ground"),
            TerrainPiece,
            Sprite::from_color(Color::srgb(0.4, 0.3, 0.2), GROUND_SIZE),
            Transform::from_xyz(0.0, GROUND_Y, 0.0),
            GlobalTransform::default(),
        ));

        for (index, translation) in PLATFORM_POSITIONS.iter().enumerate() {
            parent.spawn((
                Name::new(format!("Platform{}", index + 1)),
                TerrainPiece,
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
    roots: Query<Entity, With<TerrainRoot>>,
    pieces: Query<Entity, With<TerrainPiece>>,
    mut transitions: ResMut<UnfinishedStateTransitions>,
) {
    if pieces.is_empty() && roots.is_empty() {
        return;
    }

    transitions.add_one();

    for entity in pieces.iter() {
        commands.entity(entity).despawn();
    }

    for entity in roots.iter() {
        commands.entity(entity).despawn();
    }

    transitions.sub_one();
}
