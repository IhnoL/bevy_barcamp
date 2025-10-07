use crate::events::VerifyTerrainSpawned;
use crate::includes::{step, *};

pub fn provide_steps() -> Vec<Box<dyn TestStep>> {
    vec![step!(VerifyTerrainSpawned)]
}

pub fn handle_verify_terrain_spawned(
    _verify_event: On<VerifyTerrainSpawned>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    terrain_query: Query<&Name>,
) {
    println!("Handling VerifyTerrainSpawned");

    let terrain_found = terrain_query.iter().any(|name| name.as_str() == "Terrain");

    assert!(
        terrain_found,
        "Expected an entity named 'Terrain' to be spawned, but none was found."
    );

    unfinished_steps.complete_step();
    println!("VerifyTerrainSpawned completed.");
}
