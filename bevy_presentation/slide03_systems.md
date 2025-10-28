# Everything is a System

---

```mermaid
graph TB
    subgraph " "
        direction TB
        S1["player_move<br/>Query&lt;mut Transform, Player, Destination&gt;"]
        S2["enemy_attack_player<br/>Query&lt;mut Transform, Enemy&gt;<br/>Query&lt;Player, mut Health, Transform&gt;"]
        S3["game_over<br/>Query&lt;Player, Health&gt;"]
    end

    subgraph "Components"
        Transform[Transform]
        Player[Player]
        Enemy[Enemy]
        Destination[Destination]
        Health[Health]
    end

    Destination --> S1
    Transform --> S1
    Player --> S1
    Transform --> S2
    Enemy --> S2
    Player --> S2
    Health --> S2
    Player --> S3
    Health --> S3

    style S1 fill:#FFB6C1
    style S2 fill:#B0E0E6
    style S3 fill:#98FB98
    style Transform fill:#FFE4B5
    style Player fill:#FFE4B5
    style Enemy fill:#FFE4B5
    style Destination fill:#FFE4B5
    style Health fill:#FFE4B5
```

---

### Systems contain your game logic
- Query for Components not Entities
- Run automatically in schedules e.g. every Frame or on Startup
- Executed mostly in parallel
