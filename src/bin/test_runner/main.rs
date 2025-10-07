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
       .add_event::<StartGameStep>()
       .add_event::<QuitGameStep>()
       .add_event::<CapturePlayerPosition>()
       .add_event::<TriggerMovePlayer>()
       .add_event::<VerifyPlayerMoved>()
       .add_systems(Update, (
           send_step_from_queue,
       ).chain());

    {
        let world = app.world_mut();
        world.add_observer(movement_test::handle_capture_player_position);
        world.add_observer(movement_test::handle_move_player);
        world.add_observer(movement_test::handle_verify_player_moved);
    }

    bevy_barcamp::run(app);
}

fn send_step_from_queue(world: &mut World) {
    let mut unfinished_steps = world.resource::<UnfinishedSteps>();
    let mut test_queue = world.resource_mut::<TestStepQueue>();

    if unfinished_steps.0 == 0 {
        if let Some(step) = test_queue.steps.pop_front() {
            trigger_step_event(world, step.as_ref());
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

fn trigger_step_event(world: &mut World, step: &dyn TestStep) {
    if let Some(start_game) = step.as_any().downcast_ref::<StartGameStep>() {
        world.trigger(start_game.clone());
        return;
    }

    if let Some(quit_game) = step.as_any().downcast_ref::<QuitGameStep>() {
        world.trigger(quit_game.clone());
        return;
    }

    if let Some(capture_position) = step.as_any().downcast_ref::<CapturePlayerPosition>() {
        world.trigger(capture_position.clone());
        return;
    }

    if let Some(trigger_move) = step.as_any().downcast_ref::<TriggerMovePlayer>() {
        world.trigger(trigger_move.clone());
        return;
    }

    if let Some(verify_move) = step.as_any().downcast_ref::<VerifyPlayerMoved>() {
        world.trigger(verify_move.clone());
        return;
    }

    step.send(world);
}

