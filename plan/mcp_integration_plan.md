# MCP Integration Plan

1. **Spec Alignment with BRP**  
   - Review `spec/spec.md`, `spec/architecture.mmd`, and existing gameplay systems to confirm which controls (start, stop, move, jump) must remain accessible.  
   - Map those requirements onto Bevy Remote Protocol (BRP) capabilities exposed by `bevy_remote`, identifying any additional ECS surfaces the game must expose.  

2. **Dependencies + Plugin Wiring**  
   - Add the `bevy_remote` feature and depend on `bevy_brp_extras` so the game ships with BRP + screenshot support out of the box.  
   - Integrate `RemotePlugin::default()` and `BrpExtrasPlugin` into the Bevy app lifecycle, ensuring they activate only when the game is running (not during headless tests if that matters).  
   - Remove legacy custom MCP server scaffolding in favor of relying entirely on the BRP-driven workflow.

3. **Gameplay BRP Hooks**  
   - Ensure BRP commands can reach the systems that start/stop the game and drive player movement/jump (e.g., by exposing resources/components that BRP can mutate safely).  
   - Document the exact component/resource schemas required so remote agents can issue valid BRP operations for these controls (e.g., the `McpActionQueue` resource).  
   - Add any missing ECS adapters (events, marker components) so BRP-triggered mutations translate into existing gameplay actions.

4. **Validation + Coverage**  
   - Use `bevy_brp_mcp` to manually call BRP operations for start, stop, move, and jump, verifying they produce the expected effects in-game.  
   - Update or create automated tests (via `cargo run --bin test_runner`) that assume BRP-driven input is available, ensuring no regressions in core gameplay flows.

5. **Agent Connection Setup**  
   - Install the official MCP server (`cargo install bevy_brp_mcp`) and configure Claude Code (or other agents) with `claude mcp add --transport stdio bevy-brp "bevy_brp_mcp"`.  
   - Capture any required environment variables or project paths so agents can discover and launch the Bevy app through BRP.

6. **Documentation Updates**  
   - Revise README and `AGENTS.md` to describe the BRP-based workflow, including setup steps, required commands, and example operations for controlling the game via MCP.  
   - Document that screenshots and other extras are available through `BrpExtrasPlugin`, including any caveats for automated agents.
