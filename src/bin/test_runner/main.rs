mod events;
mod includes;
mod jump_test;
mod movement_test;
use bevy::prelude::*;
use includes::*;
use events::{QuitGameStep, StartGameStep, CapturePlayerPosition, MovePlayer, VerifyPlayerMoved};
use std::collections::VecDeque;

#[derive(Default, Resource)]
pub struct TestQueue {
    pub steps: Vec<Box<dyn TestStep>>,
}


#[derive(Default, Resource)]
pub struct TestStepQueue {
    pub queue: VecDeque<Box<dyn TestStep>>,
}


fn main() {
    let mut test_queue = TestQueue::default();

    test_queue.steps.push(step!(StartGameStep));
    test_queue.steps.extend(movement_test::provide_steps());
    test_queue.steps.push(step!(QuitGameStep));
    test_queue.steps.push(step!(StartGameStep));
    test_queue.steps.extend(jump_test::provide_steps());
    test_queue.steps.push(step!(QuitGameStep));

    run_tests(test_queue);
}

fn run_tests(test_queue: TestQueue) {
    let mut app = App::new();

    // Convert TestQueue to TestStepQueue
    let mut step_queue = TestStepQueue::default();
    for step in test_queue.steps {
        step_queue.queue.push_back(step);
    }

    app.insert_resource(step_queue)
       .init_resource::<UnfinishedSteps>()
       .init_resource::<UnfinishedSteps>()
       .add_systems(Update, (
           send_step_from_queue,
           update_test_loop,
       ).chain())
       .add_observer(movement_test::handle_capture_player_position)
       .add_observer(movement_test::handle_move_player)
       .add_observer(movement_test::handle_verify_player_moved);

    bevy_barcamp::run(app);
}

/// Producer system - sends steps from queue when there are no unfinished steps
fn send_step_from_queue(
    mut unfinished_steps: ResMut<UnfinishedSteps>,
    mut test_queue: ResMut<TestStepQueue>,
    world: &mut World,
) {
    // Only send a step if there are no unfinished steps
    if unfinished_steps.0 == 0 {
        if let Some(step) = test_queue.queue.pop_front() {
            // Increase unfinished steps counter
            unfinished_steps.0 += 1;
            println!("Sending step from queue. Unfinished steps: {}", unfinished_steps.0);

            // Send the step using its own send method
            step.send(world);
        }
    }
}


/// System to update the main loop and check conditions
fn update_test_loop(
    unfinished_steps: Res<UnfinishedSteps>,
    steps_waiting: Res<UnfinishedSteps>,
    test_queue: Res<TestStepQueue>,
) {
    // Check if all tests are completed
    if unfinished_steps.0 == 0 && test_queue.queue.is_empty() {
        println!("All tests completed!");
        std::process::exit(0);
    }
}