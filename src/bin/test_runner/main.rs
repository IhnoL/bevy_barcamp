mod events;
mod includes;
mod jump_test;
mod movement_test;
use bevy::prelude::*;
use includes::*;
use events::{QuitGameStep, StartGameStep, CapturePlayerPosition, TriggerMovePlayer, VerifyPlayerMoved};
use std::collections::VecDeque;

#[derive(Default, Resource)]
pub struct TestStepQueue {
    pub steps: VecDeque<Box<dyn TestStep>>,
}


fn main() {
    let mut test_queue = TestStepQueue::default();

    test_queue.steps.push_back(step!(StartGameStep));
    test_queue.steps.extend(movement_test::provide_steps());
    test_queue.steps.push_back(step!(QuitGameStep));
    test_queue.steps.push_back(step!(StartGameStep));
    test_queue.steps.extend(jump_test::provide_steps());
    test_queue.steps.push_back(step!(QuitGameStep));

    run_tests(test_queue);
}

fn run_tests(test_queue: TestStepQueue) {
    let mut app = App::new();

    app.insert_resource(test_queue)
       .init_resource::<UnfinishedSteps>()
       .add_systems(Update, (
           send_step_from_queue,
       ).chain())
       .add_observer(movement_test::handle_capture_player_position)
       .add_observer(movement_test::handle_move_player)
       .add_observer(movement_test::handle_verify_player_moved);

    bevy_barcamp::run(app);
}

fn send_step_from_queue(world: &mut World) {
    let mut unfinished_steps = world.resource_mut::<UnfinishedSteps>();
   // let mut test_queue = world.resource_mut::<TestStepQueue>();

    if unfinished_steps.0 == 0 {
        if let Some(step) = test_queue.steps.pop_front() {
            unfinished_steps.0 += 1;
            step.send(world);
            println!(
                "Sent step from queue. Unfinished steps: {}. Steps remaining: {}",
                unfinished_steps.0,
                test_queue.steps.len()
            );
        } else {
            println!("All tests completed!");
            std::process::exit(0);
        }
    }
}


