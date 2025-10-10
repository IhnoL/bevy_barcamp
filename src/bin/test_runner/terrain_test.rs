use bevy_barcamp::game::terrain::{TerrainPiece, TerrainRoot};

use crate::events::VerifyTerrainSpawned;
use crate::includes::{step, *};

pub fn provide_steps() -> Vec<Box<dyn TestStep>> {
    vec![step!(VerifyTerrainSpawned)]
}

pub fn handle_verify_terrain_spawned(
    _verify_event: On<VerifyTerrainSpawned>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    root_query: Query<(Entity, &Name), With<TerrainRoot>>,
    children_query: Query<&Children>,
    terrain_piece_query: Query<Entity, With<TerrainPiece>>,
) {
    println!("Handling VerifyTerrainSpawned");

    let (root_entity, _) = root_query
        .iter()
        .find(|(_, name)| name.as_str() == "Terrain")
        .unwrap_or_else(|| panic!("Terrain root entity with name 'Terrain' not found"));

    let children = children_query
        .get(root_entity)
        .unwrap_or_else(|_| panic!("Terrain root entity did not have any children"));

    let total_pieces = terrain_piece_query.iter().count();

    if total_pieces == 0 {
        panic!("No TerrainPiece components were spawned in the world");
    }

    let pieces_under_root = children
        .iter()
        .filter(|child| terrain_piece_query.get(*child).is_ok())
        .count();


    if pieces_under_root == 0 {
        panic!("Terrain root exists, but it has no children with TerrainPiece components");
    }

    unfinished_steps.complete_step();
    println!("VerifyTerrainSpawned completed.");
}
