# Repository Guidelines

## Project Structure & Module Organization
Gameplay code lives under `src/`; keep `main.rs` focused on bootstrapping by moving new systems into modules and re-exporting them with a `mod.rs`. Test diagrams are stored in `spec/` (update `architecture.mmd`, `test/test_architecture.mmd`, and `test/test_event_flowchart.mmd` alongside behavior changes). Runtime documentation sits in `docs/`, and any new assets should be placed in `assets/`. Never commit the `target/` build output.

## Build, Test, and Development Commands
Run `cargo run` from the repository root to launch the Bevy prototype. Execute `cargo test` to run unit and integration suites, and use `cargo fmt` plus `cargo clippy --all-targets --all-features` before opening a PR. For rapid feedback loops, `cargo watch -x run` is optional but encouraged.

## Coding Style & Naming Conventions
We rely on Rustfmt defaults (4-space indentation, trailing commas where valid). Modules, functions, and systems use `snake_case`; types and components use `PascalCase`; constants use `SCREAMING_SNAKE_CASE`. Keep ECS setup helpers small—for example, `fn setup_player(commands: &mut Commands)`—so `main()` remains readable. Name events precisely, only appending `Event` when deriving from `EntityEvent`.

## Testing Guidelines
Structure tests around the flows described in `spec/test/test_architecture.mmd` and the event sequence diagrams. Use `App::new()` scoped fixtures to isolate state, and name scenarios after their behavior (e.g., `movement_updates_player_position`). Every new event trigger or observer should be covered by at least one automated test; record any manual steps in the PR description.

## Commit & Pull Request Guidelines
Write commits in present tense and keep them focused (`adds player_jump_system`). Include doc and diagram updates within the same change set. Pull requests need a summary of intent, validation steps (`cargo run`, `cargo test`, relevant screenshots), and references to tracking issues when available. Highlight gameplay-visible changes with a short video or GIF when reasonable.

## Architecture & Events
The project targets Bevy 0.17’s event/message split. When introducing new flows, decide whether a buffered `Message` or immediate `Event` suits the feature, update `docs/events.md`, and reflect the design in `spec/` diagrams. Prefer `On<T>` observers and `EntityEvent` for entity-scoped interactions to keep systems predictable.
