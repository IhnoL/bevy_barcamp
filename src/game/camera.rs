use crate::game::includes::resources::UnfinishedStateTransitions;
use crate::game::includes::state::GameState;
use bevy::camera::ScalingMode;
use bevy::prelude::*;
const CAMERA_Z: f32 = 1000.0;

#[derive(Default)]
pub struct CameraPlugin;

#[derive(Component)]
pub struct GameCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Running), spawn)
            .add_systems(OnExit(GameState::Running), despawn);
    }
}

pub fn spawn(
    mut commands: Commands,
    existing_cameras: Query<(), With<GameCamera>>,
    mut transitions: ResMut<UnfinishedStateTransitions>,
) {
    if !existing_cameras.is_empty() {
        return;
    }

    transitions.add_one();

    commands.insert_resource(AmbientLight {
        color: Color::srgb(1.0, 0.98, 0.92).into(),
        brightness: 2.0,
        affects_lightmapped_meshes: true,
    });

    commands.spawn((
        Name::new("GameCamera"),
        GameCamera,
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: 800.0,
                height: 800.0,
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(0.0, 0.0, CAMERA_Z),
        GlobalTransform::default(),
    ));

    transitions.sub_one();
}

pub fn despawn(
    mut commands: Commands,
    cameras: Query<Entity, With<GameCamera>>,
    mut transitions: ResMut<UnfinishedStateTransitions>,
) {
    if cameras.is_empty() {
        return;
    }

    transitions.add_one();

    for entity in cameras.iter() {
        commands.entity(entity).despawn();
    }

    commands.remove_resource::<AmbientLight>();

    transitions.sub_one();
}
