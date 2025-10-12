# General
- game should be a 2D platformer and have a resolution of 800x800
- game should look like image /spec/layout.png (we will not implement the "Mob" for now)

# Modules
- Each module should be implemented in its own file as a bevy-plugin

## GameController(Plugin) 
- Handles creation of Game-Plugins (Player, Interaction, etc.)
- High-Level game controlling e.g. setting GameState

## Interaction(Plugin)
- movement should be triggered by left and right arrows

## Terrain(Plugin)

## Camera(Plugin)

## Player(Plugin)
- Owns Bevy-Component: Player
- Interaction Events (e.g. Move) should only be received in GameState::Running
- Is visualized by a stick man: Each body part (head, torso, arm_left, arm_right, leg_left, leg_right) is an independent line with common parent
- later on each body part should be influenced by physics on each own but still stay attached to the torso

# GameState transitions
- Transitions are only allowed to the next state and must not jump or skip 
- See the detailed flow diagram in `spec/game_state_transition.mmd` for the full transition order and responsibilities.
- GameController owns state changes; other plugins react and report their progress through `UnfinishedStateTransitions`.

## States
- Uninitialized: No entities exist in the world (anymore)
- Initializing: Entities are spawning
- Running: Entities are spawned and ready to use
- Quitting: Entities are despawning
