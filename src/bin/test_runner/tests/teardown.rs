use crate::events::{CaptureBaselineEntities, VerifyEntitiesDespawned};
use crate::includes::*;
use bevy_barcamp::game::includes::state::GameState;
use macros::step;
use std::collections::HashSet;

#[derive(Default, Resource, Clone)]
pub struct BaselineEntities {
    entities: HashSet<Entity>,
}

impl BaselineEntities {
    fn set(&mut self, snapshots: Vec<(Entity, Option<String>)>) {
        self.entities = snapshots.into_iter().map(|(entity, _)| entity).collect();
    }

    fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(&entity)
    }
}

pub fn provide_steps() -> Vec<Box<dyn TestStep>> {
    vec![step!(VerifyEntitiesDespawned)]
}

pub fn handle_capture_baseline_entities(
    _capture_event: On<CaptureBaselineEntities>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    mut baseline: ResMut<BaselineEntities>,
    named_entities: Query<(Entity, Option<&Name>)>,
) {
    println!("Handling CaptureBaselineEntities");

    let snapshots = collect_snapshots(&named_entities);
    baseline.set(snapshots);

    unfinished_steps.sub_one();
    println!("CaptureBaselineEntities completed.");
}

pub fn handle_verify_entities_despawned(
    _verify_event: On<VerifyEntitiesDespawned>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    state: Res<State<GameState>>,
    baseline: Res<BaselineEntities>,
    named_entities: Query<(Entity, Option<&Name>)>,
    ambient_light: Option<Res<AmbientLight>>,
) {
    println!("Handling VerifyEntitiesDespawned");

    if *state.get() != GameState::Uninitialized {
        panic!("VerifyEntitiesDespawned ran outside of GameState::Uninitialized");
    }

    let current_snapshots = collect_snapshots(&named_entities);

    for (entity, maybe_name) in current_snapshots {
        if baseline.contains(entity) {
            continue;
        }

        let label = maybe_name
            .as_ref()
            .map(|name| format!(" ({name})"))
            .unwrap_or_default();

        panic!(
            "Entity {entity:?}{label} remained after QuitGame but is not part of the baseline set"
        );
    }

    assert!(
        ambient_light.is_none(),
        "AmbientLight resource still present after QuitGame"
    );

    unfinished_steps.sub_one();
    println!("VerifyEntitiesDespawned completed.");
}

fn collect_snapshots(named_entities: &Query<(Entity, Option<&Name>)>) -> Vec<(Entity, Option<String>)> {
    named_entities
        .iter()
        .filter_map(|(entity, maybe_name)| {
            if entity == Entity::PLACEHOLDER {
                return None;
            }

            Some((entity, maybe_name.map(|name| name.to_string())))
        })
        .collect()
}
