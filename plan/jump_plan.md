## Implementation Plan for Jump Mechanics

Following TDD workflow as specified in project guidelines:

### Phase 1: Dependencies & Setup
1. Add the Avian physics crate to Cargo.toml
2. Configure physics plugin with gravity in main game setup

### Phase 2: Test Infrastructure Refactoring
3. Create `src/bin/test_runner/common_handlers.rs` module:
   - Move `handle_start_game` from main.rs
   - Move `handle_quit_game` from main.rs
   - Move `handle_wait_step` from main.rs
   - Move `process_wait_cycles` from main.rs
   - Move `handle_capture_player_position` from tests/movement.rs
   - This consolidates reusable test handlers in one place

4. Update `src/bin/test_runner/main.rs` to import and use handlers from common_handlers

5. Update `src/bin/test_runner/tests/movement.rs` to import handle_capture_player_position from common_handlers

### Phase 3: Test Implementation (Test-Driven)
6. Add jump test event types to `src/bin/test_runner/events.rs`:
   - `JumpPlayer`
   - `VerifyPlayerIsInTheAir`

7. Implement jump test flow in `src/bin/test_runner/tests/jump.rs`:
   - Implement `provide_steps()` with the test flow from spec
   - Create handler functions for jump-specific events
   - Reuse `handle_capture_player_position` from common_handlers

8. Register jump test observers in `src/bin/test_runner/tests.rs` TestsPlugin

9. Add jump test to main test queue in `src/bin/test_runner/main.rs`

### Phase 4: Production Code Implementation
10. Update `src/game/includes/events.rs`:
    - Add `PlayerJump` event

11. Update `src/game/player.rs`:
    - Add Avian physics components (RigidBody, Collider, Velocity)
    - Add ground detection logic
    - Implement jump system (apply upward impulse)
    - Add observer for `PlayerJump` event

12. Update `src/game/interaction.rs`:
    - Add Space key handling to trigger `PlayerJump` event

13. Update terrain with Avian colliders for ground detection

### Phase 5: Validation
14. Run tests: `cargo run --bin test_runner`
15. Fix any issues until all tests pass
16. Run game: `cargo run` to verify visually

This follows the project's TDD workflow: implement tests first, then production code.
