# Repository Guidelines

## Project Structure & Module Organization
Gameplay code lives under `src/game`.
Specification is stored in `spec/`.

## Build, Test, and Development Commands
Run `cargo run` from the repository root to launch the Bevy prototype.
Run `cargo run --bin test_runner` to execute the tests

## Testing Guidelines - Implementing Tests
Always read `src/bin/test_runner/main.rs` before implementing tests. 
Structure tests around the flows described in `spec/test/` directory.

## Architecture & Events
Before implementing any changes always check the spec/architecture.mmd and spec/spec.md.