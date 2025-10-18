mod common_handlers;
mod events;
mod includes;
mod tests;
use crate::tests::{jump_test, movement_test};
use bevy::prelude::*;
use events::{CaptureBaselineEntities, GenerateScreenshot, QuitGameStep, StartGameStep, WaitStep};
use includes::*;
use macros::step;
use std::collections::VecDeque;
use tests::{mob_test, player_test, teardown_test, terrain_test, TestsPlugin};

#[derive(Default, Resource)]
pub struct TestStepQueue {
    pub steps: VecDeque<Box<dyn TestStep>>,
}

fn main() {
    // CLI flag to control reference regeneration
    let should_generate_reference_screenshots = std::env::args()
        .any(|a| a == "--reference-screenshots");

    let mut test_queue = TestStepQueue::default();

    test_queue.steps.push_back(step!(CaptureBaselineEntities));
    test_queue.steps.push_back(step!(StartGameStep));
    test_queue.steps.push_back(step!(WaitStep { updates: 20 }));
    test_queue.steps.push_back(step!(GenerateScreenshot {
        name: "after_start".to_string(),
        is_reference: should_generate_reference_screenshots
    }));

    test_queue.steps.extend(terrain_test::provide_steps());
    test_queue.steps.extend(player_test::provide_steps());
    test_queue.steps.extend(mob_test::provide_steps());
    test_queue.steps.extend(jump_test::provide_steps());
    test_queue.steps.extend(movement_test::provide_steps());

    test_queue.steps.push_back(step!(QuitGameStep));
    test_queue.steps.extend(teardown_test::provide_steps());

    let mut app = setup_test_app(bevy_barcamp::init(App::new()), test_queue);
    app.run();
}

fn setup_test_app(mut app: App, test_queue: TestStepQueue) -> App {
    app.insert_resource(test_queue)
        .init_resource::<UnfinishedSteps>()
        .add_systems(Update, send_step_from_queue)
        .add_plugins(TestsPlugin);
    app
}

fn send_step_from_queue(world: &mut World) {
    if world.resource::<UnfinishedSteps>().is_empty() {
        if let Some(step) = world.resource_mut::<TestStepQueue>().steps.pop_front() {
            let step_name = step.to_string();
            world
                .resource_mut::<UnfinishedSteps>()
                .add(step_name.clone());
            step.send(world);
            println!("Sent step from queue: {}", step_name);
        } else {
            println!("All tests completed!");
            std::process::exit(0);
        }
    }
}

#[cfg(test)]
mod queue_tests {
    use super::*;
    use bevy::prelude::NextState;
    use bevy::state::app::StatesPlugin;
    use bevy_barcamp::game::includes::state::GameState;

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
        assert!(app.world().resource::<UnfinishedSteps>().is_empty());

        app.update();
        assert_eq!(app.world().resource::<TestStepQueue>().steps.len(), 2);
        assert_eq!(app.world().resource::<UnfinishedSteps>().0.len(), 1);

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Running);
        app.update();
        assert_eq!(app.world().resource::<TestStepQueue>().steps.len(), 1);
        assert_eq!(app.world().resource::<UnfinishedSteps>().0.len(), 1);

        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Uninitialized);
        app.update();
        assert_eq!(app.world().resource::<TestStepQueue>().steps.len(), 0);
        assert_eq!(app.world().resource::<UnfinishedSteps>().0.len(), 1);
    }
}
