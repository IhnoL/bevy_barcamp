use bevy_barcamp::game::includes::state::GameState;
use bevy_barcamp::game::mob::{Mob, MobBody, MobLeg, MobRoot};

use crate::events::VerifyMobSpawned;
use crate::includes::*;
use macros::step;

pub fn provide_steps() -> Vec<Box<dyn TestStep>> {
    vec![step!(VerifyMobSpawned)]
}

pub fn handle_verify_mob_spawned(
    _verify_event: On<VerifyMobSpawned>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    state: Res<State<GameState>>,
    root_query: Query<(Entity, &Children), (With<MobRoot>, With<Mob>)>,
    body_query: Query<(), With<MobBody>>,
    leg_query: Query<&MobLeg>,
) {
    println!("Handling VerifyMobSpawned");

    if *state.get() != GameState::Running {
        panic!("Mob verification ran outside of GameState::Running");
    }

    let mut root_iter = root_query.iter();
    let (_root_entity, children) = root_iter
        .next()
        .unwrap_or_else(|| panic!("Mob root entity with required components not found"));
    assert!(
        root_iter.next().is_none(),
        "Multiple Mob root entities found"
    );

    assert!(
        !children.is_empty(),
        "Mob root exists but does not have any children"
    );

    let mut body_found = false;
    let mut leg_indices: Vec<usize> = Vec::new();

    for child in children.iter() {
        let entity = child.clone();
        if body_query.get(entity).is_ok() {
            body_found = true;
            continue;
        }

        if let Ok(leg) = leg_query.get(entity) {
            leg_indices.push(leg.index);
        }
    }

    assert!(body_found, "Mob body child not found");

    assert_eq!(
        leg_indices.len(),
        4,
        "Expected 4 mob legs but found {}",
        leg_indices.len()
    );

    leg_indices.sort_unstable();
    assert_eq!(leg_indices, [1, 2, 3, 4], "Mob legs missing or out of order: {:?}", leg_indices);

    unfinished_steps.sub_one();
    println!("VerifyMobSpawned completed.");
}
