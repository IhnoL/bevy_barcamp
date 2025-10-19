# MCP Control Guide

This guide summarizes the practical steps for driving the prototype through the MCP interface. Treat it as a living log – append new findings as workflows evolve.

## Core Concepts
- **Action queue**: Use `brp_execute` with `method:"world.insert_resources"` and payload  
  ```json
  {"resource":"bevy_barcamp::mcp::actions::McpActionQueue","value":{"actions":[...]}}
  ```
  to push gameplay inputs. Queue multiple actions at once (e.g. `["StartGame","GetGameStatus"]`). The queue resolves one action per frame. `Move` keeps the direction held for the requested number of update cycles (default 10) and automatically releases afterward, so downstream actions execute once the movement completes.
- **World snapshot**: After queuing `GetGameStatus`, fetch the data via `brp_execute` → `method:"world.get_resources"` with `{"resource":"bevy_barcamp::mcp::actions::McpWorldState"}`. The snapshot exposes:
  - `player_position` (Vec3)
  - `game_bounds` (left/right/top/bottom)
  - `platforms` (Vec<Vec3>)
  - `mob_position` (Vec3)
  - `game_state` (`"Running"`, `"Quitting"`, etc.)
- **Hard reset**: Queue `["StopGame","StartGame"]` in one batch to force the lifecycle back through `Quitting → Uninitialized → Initializing → Running`. This guarantees a fresh spawn. For micro-adjustments afterward, use short move bursts (`steps:1`–`3`) in the desired direction.
- **Screenshots**: `brp_extras/screenshot` writes PNGs (e.g. `{"path":"<repo>/screenshots/mcp_step0_reference.png"}`) so you can validate progress visually.

## Control Flow
1. **Reset**  
   - Queue `{ "resource": "bevy_barcamp::mcp::actions::McpActionQueue", "value": { "actions": ["StopGame", "StartGame"] } }`. Confirm `game_state == "Running"` (via `GetGameStatus`) before moving.
   - Capture a baseline screenshot if you need a visual anchor.
2. **Validate bounds**  
   - Issue `GetGameStatus`; ensure `player_position` lies within `game_bounds`.
   - If outside, queue `["StartGame"]` and re-check.
3. **Walk**  
- Move right: queue `[{"Move":{"direction":"Right","steps":10}}]`. Increase or reduce `steps` (>=1) to tune the distance per burst.
- After each move, call `GetGameStatus` (or wait ~0.2s) and verify `player_position.x` stays within bounds. Remember that `GetGameStatus` actions queued behind an in-flight move run once the move finishes.
   - Mirror for left movement by setting `direction` to `"Left"`.
4. **Jump**  
   - Queue `["Jump"]`, wait a few frames (e.g. multiple `GetGameStatus` polls) and check `player_position.y`.
   - Current build: the `Jump` event can be fickle—fire multiple `["Jump"]` bursts and pair them with tiny horizontal moves (`steps:1`) while airborne to steer toward a platform. If after a couple attempts `player_position.y` never rises, issue `["StartGame"]` and try again.
5. **Mob Awareness**  
   - Use `mob_position` to monitor the enemy's location; adjust movement to avoid overlap if mechanics require it.

## Safety Checks
- Always poll `GetGameStatus` after each action batch; bail out if `game_state` changes unexpectedly.
- If snapshots report `None` for bounds or player, re-run `StartGame` (usually means entities are despawned).
- When `player_position` explodes (physics runaway), immediately queue `["StartGame"]`, wait a beat, and verify the respawn coordinates via `GetGameStatus` before resuming.
- Commands that stack large `steps` values can still overshoot. Keep bursts small and confirm via status polling before chaining additional moves.

## Troubleshooting
- **Queue appears ignored**: ensure the JSON sets `actions` to an array of enum instances. Enum variants are in PascalCase.
- **No state updates**: verify the world resource name matches `bevy_barcamp::mcp::actions::McpWorldState`.
- **Out-of-range coordinates**: immediately restart with `["StartGame"]` before issuing additional moves.
- **Jump still flat**: if repeated jump bursts fail to increase `player_position.y`, restart with `["StartGame"]` and double-check that gravity and ground contact look correct before retrying the sequence.

## Worked Example – Platform Approach
- Start with `["StartGame"]`, wait ~0.5s, capture `screenshots/mcp_step0_reference.png`, and confirm via `GetGameStatus` that `player_position` is near `(-320, -290, 0.2)`.
- Advance right by repeating `{"Move":{"direction":"Right","steps":10}}`; poll `GetGameStatus` between steps to log the incremental x-coordinates (e.g. -320 → -215). Snapshot `screenshots/mcp_step1_ground_near_platform.png` once aligned under the left platform.
- Issue `["Jump"]`, then chain short `Move` bursts (`steps:1`–`2` in the jump direction) while airborne to drift onto the platform. Repeat the jump + drift combo until `player_position.y` reflects the new height, then capture `screenshots/mcp_step2_on_platform.png`.
- If a movement burst launches the player far outside bounds, immediately enqueue `["StartGame"]`, wait a few frames, and resume with smaller `steps` values to stay within bounds.
- After landing, re-run `GetGameStatus` to confirm the state snapshot before any automated tests (`test_runner`) reuse the session; the runner leaves the game running, so the agent can continue from the confirmed coordinates.

## Session Log – Middle Platform (2025-10-19)
- Baseline: queue `["GetGameStatus"]` after spawning to confirm the default ground position (`(-320, -290, 0.2)`) and bounds. Walk right in controlled bursts (`steps:10 → 8 → 5 → 3`) until the player sits beneath the middle platform (`x ≈ -12`). Poll status between bursts to keep the mob and player inside bounds.
- Gain altitude by chaining a jump with two short right holds:
  ```json
  {
    "actions": [
      "Jump",
      {"Move":{"direction":"Right","steps":8}},
      {"Move":{"direction":"Right","steps":6}},
      "GetGameStatus"
    ]
  }
  ```
  This sequence carried the player to `player_position ≈ (248.38, -40.47, 0.2)` while the queue finished the release automatically.
- From the right-hand ground position (`x ≈ 320`), fire one more burst to arc back over the middle platform:
  ```json
  {
    "actions": [
      "Jump",
      {"Move":{"direction":"Left","steps":30}}
    ]
  }
  ```
  The long left hold while airborne dropped the avatar onto the middle platform at `player_position ≈ (27.11, -80.00, 0.2)`.
- Finish with `["GetGameStatus"]` to verify `game_state == "Running"` and capture `screenshots/mcp_debug.png` via `brp_extras/screenshot`. Leave the process running if a human wants to inspect the live state.

## Worked Example – Three Platform Sweep
- Reset and confirm ground start (`GetGameStatus` ≈ `(-320, -290, 0.2)`), then grab a baseline screenshot such as `screenshots/mcp_platforms_step0_ground.png`.
- Left platform: chain `{"Move":{"direction":"Right","steps":6}}` twice followed by a `steps:2` burst to settle around `x ≈ -195`. Fire a `["Jump"]` and add one or two `steps:1` nudges while airborne to stick the landing; capture `screenshots/mcp_platforms_step1_left.png`.
- Middle platform: from the left perch, take a short right burst (`steps:4`) to drop off the edge, then pause until back on ground level. Walk to the center with alternating `steps:5` right moves, jump, and apply mid-air nudges until the status snapshot shows `y` near the middle platform height. Save `screenshots/mcp_platforms_step2_middle.png`.
- Right platform: after dismounting the middle platform, repeat controlled right bursts (start with `steps:6`, then `steps:3`) to align under the final platform. Use a jump plus quick right nudges to reach it and record `screenshots/mcp_platforms_step3_right.png`.
- Throughout the sweep, poll `GetGameStatus` after every burst to ensure `game_state == "Running"` and the coordinates remain inside the reported bounds before continuing.
