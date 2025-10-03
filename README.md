# bevy_barcamp

This project is built with [Rust](https://www.rust-lang.org/learn/get-started) and powered by [Bevy 0.17](https://bevyengine.org/learn/book/getting-started/), the latest release of the engine.

## Getting Started

- Install the Rust toolchain by following the official setup guide linked above.
- Run `cargo run` from the repository root to launch the application.

## Bevy 0.17 Events and Messages

We adopt Bevy 0.17's new separation between **events** and **messages**, which gives us static guarantees about how observers run and prevents misuse of the APIs. Every event now specifies an associated trigger, and targeted entity events derive from `EntityEvent`.

## Documentation

- Architecture overview: [docs/architecture.mmd](docs/architecture.mmd)
- Test architecture: [docs/test_architecture.mmd](docs/test_architecture.mmd)
- Event system guide: [docs/events.md](docs/events.md)
