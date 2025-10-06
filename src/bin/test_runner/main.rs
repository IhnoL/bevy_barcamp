mod events;
mod includes;
mod jump_test;
mod movement_test;
use bevy::prelude::*;
use includes::*;
use events::{QuitGameStep, StartGameStep};

#[derive(Default, Resource)]
pub struct TestQueue {
    pub steps: Vec<Box<dyn TestStep>>,
}

fn main() {
    let mut test_queue = TestQueue::default();

    // Add quit and start step to queue
    test_queue.steps.push(step!(QuitGameStep));
    test_queue.steps.push(step!(StartGameStep));

    // Fetch test steps from movement_test and add to queue
    let movement_steps = movement_test::provide_steps();
    test_queue.steps.extend(movement_steps);

    // Another quit and start step
    test_queue.steps.push(step!(QuitGameStep));
    test_queue.steps.push(step!(StartGameStep));

    // Fetch jump test
    let jump_steps = jump_test::provide_steps();
    test_queue.steps.extend(jump_steps);

    // Final quit
    test_queue.steps.push(step!(QuitGameStep));

    // Run the tests
    run_tests(test_queue);
}

fn run_tests(test_queue: TestQueue) {
    // Create the app and run test execution
    let mut app = App::new();
    app.insert_resource(test_queue);
    app.run();
}
