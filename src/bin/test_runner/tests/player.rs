use std::collections::HashSet;

use bevy_barcamp::game::includes::state::GameState;
use bevy_barcamp::game::player::{BodyPart, Player, PlayerBodyPart, PlayerRoot};

use crate::events::VerifyPlayerSpawned;
use crate::includes::*;
use macros::step;

pub fn provide_steps() -> Vec<Box<dyn TestStep>> {
    vec![step!(VerifyPlayerSpawned)]
}

pub fn handle_verify_player_spawned(
    _verify_event: On<VerifyPlayerSpawned>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    state: Res<State<GameState>>,
    root_query: Query<Entity, (With<Player>, With<PlayerRoot>)>,
    children_query: Query<&Children>,
    body_part_query: Query<(&PlayerBodyPart, &ChildOf)>,
) {
    println!("Handling VerifyPlayerSpawned");

    if *state.get() != GameState::Running {
        panic!("Player verification ran outside of GameState::Running");
    }

    let mut root_iter = root_query.iter();
    let root_entity = root_iter
        .next()
        .unwrap_or_else(|| panic!("Player root entity with required components not found"));
    assert!(
        root_iter.next().is_none(),
        "Multiple Player root entities found"
    );

    let children = children_query
        .get(root_entity)
        .unwrap_or_else(|_| panic!("Player root entity did not have any children"));

    assert!(
        !children.is_empty(),
        "Player root exists but does not have any body part children"
    );

    let mut kinds = HashSet::new();
    let mut attached_part_count = 0usize;
    for (body_part, child_of) in body_part_query.iter() {
        if child_of.parent() == root_entity {
            kinds.insert(body_part.kind);
            attached_part_count += 1;
        }
    }

    assert_eq!(
        attached_part_count,
        children.len(),
        "Player root children count ({}) does not match number of PlayerBodyPart attachments ({attached_part_count})",
        children.len()
    );

    let expected_parts = [
        BodyPart::Head,
        BodyPart::Torso,
        BodyPart::ArmLeft,
        BodyPart::ArmRight,
        BodyPart::LegLeft,
        BodyPart::LegRight,
    ];
    for expected in expected_parts.iter() {
        assert!(
            kinds.contains(expected),
            "Player is missing required body part: {:?}",
            expected
        );
    }

    assert_eq!(
        kinds.len(),
        expected_parts.len(),
        "Player spawned with unexpected number of unique body parts: {:?}",
        kinds
    );

    unfinished_steps.sub_one();
    println!("VerifyPlayerSpawned completed.");
}
