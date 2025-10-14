mod events;
mod includes;
mod tests;
use crate::tests::movement;
use bevy::prelude::*;
use bevy_barcamp::game::includes::state::GameState;
use events::{CaptureBaselineEntities, QuitGameStep, StartGameStep, WaitStep};
use includes::*;
use macros::step;
use std::collections::VecDeque;
use tests::{TestsPlugin, mob, player, teardown, terrain};

#[derive(Default, Resource)]
pub struct TestStepQueue {
    pub steps: VecDeque<Box<dyn TestStep>>,
}

fn main() {
    let mut test_queue = TestStepQueue::default();

    test_queue.steps.push_back(step!(CaptureBaselineEntities));
    test_queue.steps.push_back(step!(StartGameStep));
    test_queue.steps.extend(terrain::provide_steps());
    test_queue.steps.extend(player::provide_steps());
    test_queue.steps.extend(mob::provide_steps());
    test_queue.steps.extend(movement::provide_steps());
    test_queue.steps.push_back(step!(QuitGameStep));
    test_queue.steps.extend(teardown::provide_steps());

    let base_app = bevy_barcamp::run(App::new());
    let mut app = setup_test_app(base_app, test_queue);
    app.run();
}

fn setup_test_app(mut app: App, test_queue: TestStepQueue) -> App {
    app.insert_resource(test_queue)
        .init_resource::<UnfinishedSteps>()
        .init_resource::<PendingWaitStep>()
        .add_systems(Update, (process_wait_cycles, send_step_from_queue))
        .add_systems(OnEnter(GameState::Running), handle_start_game)
        .add_systems(OnEnter(GameState::Uninitialized), handle_quit_game)
        .add_observer(handle_wait_step)
        .add_plugins(TestsPlugin);
    app
}

fn send_step_from_queue(world: &mut World) {
    if world.resource::<UnfinishedSteps>().0 == 0 {
        if let Some(step) = world.resource_mut::<TestStepQueue>().steps.pop_front() {
            world.resource_mut::<UnfinishedSteps>().add_one();
            step.send(world);
            println!("Sent step from queue.",);
        } else {
            println!("All tests completed!");
            std::process::exit(0);
        }
    }
}

fn handle_start_game(mut unfinished_steps: ResMut<UnfinishedSteps>) {
    unfinished_steps.sub_one();
    println!("StartGameStep completed.");
}

fn handle_quit_game(mut unfinished_steps: ResMut<UnfinishedSteps>) {
    unfinished_steps.sub_one();
    println!("QuitGameStep completed.");
}

fn handle_wait_step(wait_step: On<WaitStep>, mut pending: ResMut<PendingWaitStep>) {
    println!("Handling WaitStep waiting");
    pending.wait_cycles = Some(wait_step.updates);
}

fn process_wait_cycles(
    mut pending: ResMut<PendingWaitStep>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
) {
    if let Some(wait_cycles) = pending.wait_cycles.as_mut() {
        println!("Still waiting..");
        *wait_cycles = wait_cycles.saturating_sub(1);
        if *wait_cycles == 0 {
            println!("WaitStep completed.");
            pending.wait_cycles = None;
            unfinished_steps.sub_one();
        }
    }
}

#[cfg(test)]
mod queue_tests {
    use super::*;
    use bevy::prelude::NextState;
    use bevy::state::app::StatesPlugin;

    #[test]
    fn test_queue_processes_steps_sequentially() {
        // Setup headless game
        use bevy::MinimalPlugins;
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(StatesPlugin);
        app.init_state::<GameState>();
        app.world_mut().init_resource::<NextState<GameState>>();

        // Setup test
        let mut test_queue = TestStepQueue::default();
        test_queue.steps.push_back(step!(StartGameStep));
        test_queue.steps.push_back(step!(QuitGameStep));
        test_queue.steps.push_back(step!(StartGameStep));
        app = setup_test_app(app, test_queue);

        // Run test
        assert_eq!(app.world().resource::<TestStepQueue>().steps.len(), 3);
        assert_eq!(app.world().resource::<UnfinishedSteps>().0, 0);

        app.update();
        assert_eq!(app.world().resource::<TestStepQueue>().steps.len(), 2);
        assert_eq!(app.world().resource::<UnfinishedSteps>().0, 1);

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Running);
        app.update();
        assert_eq!(app.world().resource::<TestStepQueue>().steps.len(), 1);
        assert_eq!(app.world().resource::<UnfinishedSteps>().0, 1);

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Uninitialized);
        app.update();
        assert_eq!(app.world().resource::<TestStepQueue>().steps.len(), 0);
        assert_eq!(app.world().resource::<UnfinishedSteps>().0, 1);
    }
}
