use bevy_barcamp::game::includes::state::GameState;
use bevy_barcamp::game::mob::{Mob, MobBodyPart, MobPart};

use crate::events::VerifyMobSpawned;
use crate::includes::*;
use macros::step;

pub fn provide_steps() -> Vec<Box<dyn TestStep>> {
    vec![step!(VerifyMobSpawned)]
}

pub fn handle_verify_mob_spawned(
    _verify_event: On<VerifyMobSpawned>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    mob_query: Query<(Entity, &Children), With<Mob>>,
    mob_body_part_query: Query<(&MobBodyPart, &ChildOf)>,
) {
    let mut mob_iter = mob_query.iter();
    let (root_entity, children) = mob_iter
        .next()
        .expect("Mob root entity with required components not found");
    assert!(
        mob_iter.next().is_none(),
        "Multiple Mob root entities found"
    );

    assert!(
        !children.is_empty(),
        "Mob root exists but does not have any children"
    );

    let mut torso_count = 0usize;
    let mut leg_count = 0usize;
    let mut attached_part_count = 0usize;

    for (part, child_of) in mob_body_part_query.iter() {
        if child_of.parent() == root_entity {
            attached_part_count += 1;
            match part.kind {
                MobPart::Torso => torso_count += 1,
                MobPart::Leg => leg_count += 1,
            }
        }
    }

    assert_eq!(
        attached_part_count,
        children.len(),
        "Mob root children count ({}) does not match number of MobBodyPart attachments ({attached_part_count})",
        children.len()
    );

    assert_eq!(torso_count, 1, "Mob spawned with {torso_count} torsos instead of 1");
    assert_eq!(leg_count, 4, "Mob spawned with {leg_count} legs instead of 4");

    unfinished_steps.sub_one();
    println!("VerifyMobSpawned completed.");
}
