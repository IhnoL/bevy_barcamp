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
- Must always be in the Order: Uninitialized -> Initializing -> Running -> Quitting -> Uninitialized

## On Start/Quit -Event 
- GameController sets GameState:: Initializing/Quitting 
- Each on_init()/on_quit() is called and increases the CompletionCounter; decreasing it again on finishing 
- GameController waits for CompletionCounter to reach 0 and then sets GameState:: Running/Uninitialized