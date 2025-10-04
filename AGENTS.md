# Repository Guidelines

Review the [README](README.md) for a high-level project overview before diving into the process details below.

## Project Structure & Module Organization
The gameplay prototype currently resides in `src/main.rs`; break out new systems into modules under `src/` and re-export them through a `mod.rs` to keep startup code tidy. Assets and additional Bevy config should live under an `assets/` directory at the repo root (create it if your feature needs one). Contributor-focused diagrams live under `spec/` (`architecture.mmd`, `test/test_architecture.mmd`, `test/test_event_flowchart.mmd`) and should be updated alongside code changes. Event details live in `docs/events.md`. Build artifacts appear in `target/`; never commit that directory.

## Build, Test, and Development Commands
Use `cargo run` to launch the Bevy app and validate runtime behavior. `cargo test` executes unit and integration tests, including any `#[cfg(test)]` modules inside `src/`. Format the codebase with `cargo fmt` before opening a review, and run `cargo clippy --all-targets --all-features` to catch Bevy-specific lints. When iterating quickly, `cargo watch -x run` is recommended but optional.

## Coding Style & Naming Conventions
Stick to Rustfmt defaults (4-space indentation, trailing commas where valid) and address all formatter diffs. Modules and functions use `snake_case`, types and systems use `PascalCase`, and constants use `SCREAMING_SNAKE_CASE`. Group ECS setup in dedicated functions (e.g., `fn setup_player(commands: &mut Commands)`) to keep `main()` legible. Prefer descriptive event names ending in `Event` only when derived from `EntityEvent`; otherwise keep them concise (`Move`, `Restart`).

## Testing Guidelines
Follow the event-driven flow outlined in `spec/test/test_architecture.mmd` and the queue lifecycle described in `spec/test/test_event_flowchart.mmd`: compose scenario controllers that trigger events and assert observer effects. Name tests with the behavior under test, e.g., `movement_updates_player_position`. Use `App::new()` scoped setups to limit state leakage between cases. Aim to cover new event triggers and observers with at least one unit test; document non-trivial manual steps in the PR description.

## Commit & Pull Request Guidelines
Keep commits focused and written in present tense, mirroring the existing history (`adds test_architecture`). Include diagrams or doc updates that reflect code changes. Pull requests need a concise summary, reproduction or validation steps (`cargo run`, `cargo test` outputs), and links to relevant issues. Attach screenshots or screen recordings when a change alters visuals or input handling.

## Bevy Events & Messages
Bevy 0.17 separates immediate `Event` observers from buffered `Message` channels. When introducing new gameplay flows, decide which interface you need and update `docs/events.md` with any new trigger patterns. Leverage `On<T>` observers and `EntityEvent` for entity-targeted interactions, and mirror any architectural changes in the diagrams under `spec/`.
