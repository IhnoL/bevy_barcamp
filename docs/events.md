# Events in Bevy 0.17

Bevy 0.17 reframes the `Event` trait around *triggers*. Instead of each event carrying propagation config, the trait now looks like this:

```rust
trait Event {
    type Trigger<'a>: Trigger<Self>;
}
```

Each event type declares which trigger drives `world.trigger()`. This gives us compile-time guarantees about where observers run and the data they receive.

## Observers

Observers are now written against `On<T>` instead of `Trigger<T>`:

```rust
#[derive(Event)]
struct GameOver {
    score: u32,
}

world.add_observer(|game_over: On<GameOver>| {
    info!("Game over! Score: {}", game_over.score);
});

world.trigger(GameOver { score: 100 });
```

The `On` parameter is conceptually “the event itself,” which encourages naming observers after the event.

## Entity Events

If an event needs to target entities, derive `EntityEvent`:

```rust
#[derive(EntityEvent)]
struct Click {
    entity: Entity,
}

world.trigger(Click { entity });
```

This enables entity-scoped observers:

```rust
world.entity_mut(entity).observe(|mut click: On<Click>| {
    click.propagate(false); // optional propagation control
});
```

Propagation is opt-in via `#[entity_event(propagate)]`, and every entity event exposes the original target through `On::original_event_target()` when propagation is enabled.

## Messages vs. Events

Events are now strictly for observer-based flows. Buffered communication moved to the new `Message` trait, using `MessageWriter`/`MessageReader`. Only implement `Message` when you need buffered delivery; implement `Event` when something must trigger observers immediately.

## How We Use This

- All new events in this codebase derive either `Event` or `EntityEvent` and use the default triggers unless a custom trigger is required.
- Observers use the `On<T>` signature for clarity and type-safety.
- Buffered channels (game logs, analytics, etc.) should be modeled as messages, not events.

Refer back to this guide when adding new gameplay systems so everything stays aligned with Bevy 0.17 semantics.
