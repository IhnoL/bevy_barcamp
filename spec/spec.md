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
- Is visualized by a stick man: Each body part (head, torso, arms, legs) is an independent line with common parent

# GameState transitions
- Must always be in the Order: Uninitialized -> Initializing -> Running -> Quitting -> Uninitialized. States can not be skipped
- State transition can only be executed (mutable) by the GameController; Other modules are only listening
- Modules that are reacting on state-change, increases the UnfinishedStateTransitions counter and decrease it on completion
- The GameController waits for all transitions to complete before changing the state to the next
- The GameController completes changing the states to the next until the Target-State is reached
- After receiving a StartGame Event the Target-State is set to "Running" and for QuitGame it's "Uninitialized"

## States
- Uninitialized: No entities exist in the world (anymore)
- Initializing: Entities are spawning
- Running: Entities are spawned and ready to use
- Quitting: Entities are despawning