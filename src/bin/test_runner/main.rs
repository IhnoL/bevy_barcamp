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

    test_queue.steps.push(step!(StartGameStep));

    test_queue.steps.extend( movement_test::provide_steps());

    test_queue.steps.push(step!(QuitGameStep));
    test_queue.steps.push(step!(StartGameStep));

    test_queue.steps.extend(jump_test::provide_steps());

    test_queue.steps.push(step!(QuitGameStep));

    run_tests(test_queue);
}

fn run_tests(test_queue: TestQueue) {
    let mut app = App::new();
    app.insert_resource(test_queue);
    bevy_barcamp::run(app);
}
