# bevy_barcamp

This project is built with [Rust](https://www.rust-lang.org/learn/get-started) and powered by [Bevy 0.17](https://bevy.org/learn/quick-start/introduction/), the latest release of the engine. 
It can be used as a reference implementation for an effective Vibe-Coding environment to develop a game.
Focus is to have a strong Testing-Infrastructure, so that the AI-Coding Agent is able to test the game on its own without user validation on each step.

## Getting Started

- Install the Rust toolchain by following the official setup guide linked above.
- Install the Programming environment of your choice. Recommended: RustRover, VSCode or Vibe-IDEs like Cursor
- Run `cargo run` from the repository root to launch the application.
- Run `cargo run --bin test_runner` to execute the automated test suite.
- Generate reference screenshots: `cargo run --bin test_runner -- --reference-screenshots`
- Install a Vibe-Coding Tool of your choice. Recommended: Claude-Code, Codex

## Documentation

- Architecture overview: [spec/architecture.mmd](spec/architecture.mmd)
- Test architecture: [spec/test/test_architecture.mmd](spec/test/test_architecture.mmd)
- Test event flow: [spec/test/test_event_flowchart.mmd](spec/test/test_event_flowchart.mmd)
- Agents context: [AGENTS.md](AGENTS.md)
