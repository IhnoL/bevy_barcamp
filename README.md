# bevy_barcamp

This project is built with [Rust](https://www.rust-lang.org/learn/get-started) and powered by [Bevy 0.17](https://bevyengine.org/learn/book/getting-started/), the latest release of the engine. New contributors should also read the [contributor guidelines](AGENTS.md) to stay aligned with the project's workflows.

## Getting Started

- Install the Rust toolchain by following the official setup guide linked above.
- Run `cargo run` from the repository root to launch the application.
- Run `cargo run --bin test_runner` to execute the automated test suite.

## Bevy 0.17 Events and Messages

We adopt Bevy 0.17's new separation between **events** and **messages**, which gives us static guarantees about how observers run and prevents misuse of the APIs. Every event now specifies an associated trigger, and targeted entity events derive from `EntityEvent`.

## Documentation

- Architecture overview: [spec/architecture.mmd](spec/architecture.mmd)
- Test architecture: [spec/test/test_architecture.mmd](spec/test/test_architecture.mmd)
- Test event flow: [spec/test/test_event_flowchart.mmd](spec/test/test_event_flowchart.mmd)
- Event system guide: [docs/events.md](docs/events.md)
- Contributor guidelines: [AGENTS.md](AGENTS.md)
