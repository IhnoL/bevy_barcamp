mod common_handlers;
mod events;
mod includes;
mod tests;
use crate::tests::{jump, movement};
use bevy::prelude::*;
use events::{CaptureBaselineEntities, QuitGameStep, StartGameStep};
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
    test_queue.steps.extend(jump::provide_steps());
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
        .add_systems(Update, send_step_from_queue)
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

#[cfg(test)]
mod queue_tests {
    use super::*;
    use bevy_barcamp::game::includes::state::GameState;
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
