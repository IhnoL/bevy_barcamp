# Events

---

```mermaid
sequenceDiagram
    participant I as Input System
    participant C as (mut) Commands
    participant O1 as Observer: on_jump
    participant O2 as Observer: on_move
    participant O3 as Observer: game_over

    I->>C: PlayerJump
    C->>O1: On<PlayerJump>

    I->>C: PlayerMove(Direction)
    C->>O2: On<PlayerMove>

    C->>O3: On<Changed, Health>
```

---

### Decouple everything
- Events trigger observers immediately
- Events can carry data (e.g., direction)
- Observers are normal bevy systems
- Observers can also listen for Component changed / added / removed
- Messages are a more performant alternative for logic that must not execute immediately
