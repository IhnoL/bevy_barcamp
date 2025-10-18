# General
- game should be a 2D platformer and have a resolution of 800x800
- game should look like image /spec/layout.png (we will not implement the "Mob" for now)
- game uses the Avian physics engine (https://github.com/Jondolf/avian) for gravity and vertical movement

# Modules
- Each module should be implemented in its own file as a bevy-plugin

## GameController(Plugin) 
- Handles creation of Game-Plugins (Player, Interaction, etc.)
- High-Level game controlling e.g. setting GameState

## Interaction(Plugin)
- all interactions trigger bevy .17 events and not messages
- movement should not be physics based: Triggered by WASD (e.g. left, right, ladder-up, ladder-down)
- jump should be physics based: Triggered by Space key an upward velocity impulse is applied, with falling handled by physics 

## Terrain(Plugin)

## Camera(Plugin)

## Player(Plugin)
- Owns Bevy-Component: Player
- Interaction Events (e.g. Move) should only be received in GameState::Running
- Is visualized by a stick man: Each body part ( torso, arm_left, arm_right, leg_left, leg_right) is an independent line with common parent
- The head is not a stick but a smiling face from  `assets/textures/lol.png`
- later on each body part should be influenced by physics on each own but still stay attached to the torso
- The player has a component "Grounded" which is removed when he jumps or falls and re-attached when grounded again

# GameState transitions
- Transitions are only allowed to the next state and must not jump or skip 
- See the detailed flow diagram in `spec/game_state_transition.mmd` for the full transition order and responsibilities.
- GameController owns state changes; other plugins react and report their progress through `UnfinishedStateTransitions`.

## States
- Uninitialized: No entities exist in the world (anymore)
- Initializing: Entities are spawning
- Running: Entities are spawned and ready to use
- Quitting: Entities are despawning

## MCP Integration
- The game will expose Bevy Remote Protocol support (`RemotePlugin`, `BrpExtrasPlugin`) so external MCP servers such as `bevy_brp_mcp` can drive gameplay features. 
- All remote operations must continue to flow through Bevy-safe ECS interfaces (commands, resources) to avoid unsafe world access.
