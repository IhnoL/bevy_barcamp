# Entity Component System (ECS)

---

```mermaid
graph TB
    subgraph "Game World"
        E1[Entity: A]
        E2[Entity: B]
        E3[Entity: C]
    end

    subgraph "__________Components"
        Transform[Transform<br/>position, rotation]
        Health[Health<br/>current, max]
        Player[Player]
        Enemy[Enemy]
        Collider[Collider<br/>shape, size]
        Platform
    end

    E1 -.->|has| Player
    E1 -.-> Health
    E1 -.-> Transform
    E1 -.-> Collider

    E2 -.-> Transform
    E2 -.-> Health
    E2 -.-> Enemy
    E2 -.-> Collider

    E3 -.-> Transform
    E3 -.-> Collider
    E3 -.-> Platform

    style E1 fill:#98FB98
    style E2 fill:#FFB347
    style E3 fill:#DDA0DD
```

---

### Composition Over Inheritance
- Entities are objects in the world with a unique ID
- Components are attached to entities and may hold data
- Mix and match components to define behavior
- No rigid class hierarchies
