# Repository Guidelines

## Project Structure & Module Organization
Gameplay code lives under `src/game`.
Specification is stored in `spec/`.

## Build, Test, and Development Commands
Run `cargo run` from the repository root to launch the Bevy prototype.
Run `cargo run --bin test_runner` to execute the tests after each change !!!
To regenerate reference screenshots used by image comparisons, run:
`cargo run --bin test_runner -- --reference-screenshots`

## Testing Guidelines - Implementing Tests
Always read `src/bin/test_runner/main.rs` before implementing tests.
Structure tests around the flows described in `spec/test/` directory.

## Architecture
Before implementing any changes always check the spec/architecture.mmd and spec/spec.md.

## Workflow
Work Test-Driven if possible.
Implement the functional tests in `src/bin/test_runner/` first.
For more complex algorithms also implement unit tests but not for simple code that is already tested by the functional test.