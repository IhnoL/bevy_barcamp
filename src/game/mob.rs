use bevy::prelude::*;

use crate::game::includes::resources::UnfinishedStateTransitions;
use crate::game::includes::state::GameState;

const MOB_TEXTURE_PATH: &str = "textures/mob.png";
const MOB_POSITION: Vec3 = Vec3::new(240.0, -300.0, 0.2);
const MOB_BODY_OFFSET: Vec3 = Vec3::new(0.0, 32.0, 0.05);
const MOB_LEG_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const MOB_LEG_Z_OFFSET: f32 = 0.0;

#[derive(Clone, Copy)]
struct LegSpec {
    offset: Vec3,
    size: Vec2,
    rotation_radians: f32,
}

const MOB_LEG_SPECS: [LegSpec; 4] = [
    LegSpec {
        offset: Vec3::new(-32.0, -36.0, MOB_LEG_Z_OFFSET),
        size: Vec2::new(64.0, 4.0),
        rotation_radians: std::f32::consts::PI / 5.0,
    },
    LegSpec {
        offset: Vec3::new(-10.0, -40.0, MOB_LEG_Z_OFFSET),
        size: Vec2::new(60.0, 4.0),
        rotation_radians: std::f32::consts::FRAC_PI_8,
    },
    LegSpec {
        offset: Vec3::new(10.0, -40.0, MOB_LEG_Z_OFFSET),
        size: Vec2::new(60.0, 4.0),
        rotation_radians: -std::f32::consts::FRAC_PI_8,
    },
    LegSpec {
        offset: Vec3::new(32.0, -36.0, MOB_LEG_Z_OFFSET),
        size: Vec2::new(64.0, 4.0),
        rotation_radians: -std::f32::consts::PI / 5.0,
    },
];

#[derive(Default)]
pub struct MobPlugin;

#[derive(Component)]
pub struct Mob;

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MobBodyPart {
    pub kind: MobPart,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MobPart {
    Torso,
    Leg,
}

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Initializing), spawn)
            .add_systems(OnEnter(GameState::Quitting), despawn);
    }
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing: Query<(), With<Mob>>,
    mut transitions: ResMut<UnfinishedStateTransitions>,
) {
    if !existing.is_empty() {
        return;
    }

    transitions.add_one();

    let texture = asset_server.load(MOB_TEXTURE_PATH);

    let mut root = commands.spawn((
        Name::new("Mob"),
        Mob,
        Transform::from_translation(MOB_POSITION),
        GlobalTransform::default(),
    ));

    root.with_children(|parent| {
        parent.spawn((
            Name::new("mob-torso"),
            MobBodyPart {
                kind: MobPart::Torso,
            },
            Sprite::from_image(texture.clone()),
            Transform::from_translation(MOB_BODY_OFFSET),
            GlobalTransform::default(),
        ));

        for (index, spec) in MOB_LEG_SPECS.iter().enumerate() {
            parent.spawn((
                Name::new(format!("mob-leg-{index}")),
                MobBodyPart {
                    kind: MobPart::Leg,
                },
                Sprite::from_color(MOB_LEG_COLOR, spec.size),
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
    roots: Query<(Entity, Option<&Children>), With<Mob>>,
    mut transitions: ResMut<UnfinishedStateTransitions>,
) {
    if roots.is_empty() {
        return;
    }

    transitions.add_one();

    for (entity, children) in roots.iter() {
        if let Some(children) = children {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }

        commands.entity(entity).despawn();
    }

    transitions.sub_one();
}
