use bevy_barcamp::game::terrain::{TerrainPiece, TerrainRoot};

use crate::events::VerifyTerrainSpawned;
use crate::includes::*;
use bevy_barcamp::game::includes::state::GameState;
use macros::step;

pub fn provide_steps() -> Vec<Box<dyn TestStep>> {
    vec![step!(VerifyTerrainSpawned)]
}

pub fn handle_verify_terrain_spawned(
    _verify_event: On<VerifyTerrainSpawned>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    game_state: Res<State<GameState>>,
    terrain_root_query: Query<(Entity, &Name), With<TerrainRoot>>,
    children_query: Query<&Children>,
    terrain_piece_query: Query<Entity, With<TerrainPiece>>,
) {
    println!("Handling VerifyTerrainSpawned");

    if *game_state.get() != GameState::Running {
        panic!("Terrain verification ran outside of GameState::Running");
    }

    let (root_entity, _) = terrain_root_query
        .iter()
        .find(|(_, name)| name.as_str() == "Terrain")
        .unwrap_or_else(|| panic!("Terrain root entity with name 'Terrain' not found"));

    let children = children_query
        .get(root_entity)
        .unwrap_or_else(|_| panic!("Terrain root entity did not have any children"));

    let total_pieces = terrain_piece_query.iter().count();
    assert!(
        total_pieces > 0,
        "No TerrainPiece components were spawned in the world"
    );

    let pieces_under_root = children
        .iter()
        .filter(|child| terrain_piece_query.get(*child).is_ok())
        .count();
    assert!(
        pieces_under_root == total_pieces,
        "Terrain root exists, but its TerrainPiece children ({}) do not match total TerrainPiece entities ({})",
        pieces_under_root,
        total_pieces
    );

    unfinished_steps.sub_one();
    println!("VerifyTerrainSpawned completed.");
}
