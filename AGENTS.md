# Repository Guidelines

## Project Structure & Module Organization
Gameplay code lives under `src/game`.
Specification is stored in `spec/`.

## Build, Test, and Development Commands
Run `cargo run` from the repository root to launch the Bevy prototype. 

## Testing Guidelines
Code lives in ´src/bin/test_runner´ Structure tests around the flows described in `spec/test/` directory.

## Architecture & Events
The project targets Bevy 0.17’s event/message split. When introducing new flows, decide whether a buffered `Message` or immediate `Event` suits the feature, update `docs/events.md`, and reflect the design in `spec/` diagrams. Prefer `On<T>` observers and `EntityEvent` for entity-scoped interactions to keep systems predictable.
Never use struct and class like implementation if not necessary. Instead, use simple functions if possible and store data in the bevy way.