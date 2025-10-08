use bevy::camera::ScalingMode;
use bevy::prelude::*;

use super::state::GameState;

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

pub fn spawn(mut commands: Commands, existing_cameras: Query<(), With<GameCamera>>) {
    if !existing_cameras.is_empty() {
        return;
    }

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
}

pub fn despawn(mut commands: Commands, cameras: Query<Entity, With<GameCamera>>) {
    for entity in cameras.iter() {
        commands.entity(entity).despawn();
    }
}
