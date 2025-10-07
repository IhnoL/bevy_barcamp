mod events;
mod includes;
mod jump_test;
mod movement_test;
use bevy::prelude::*;
use events::{
    CapturePlayerPosition, QuitGameStep, StartGameStep, TriggerMovePlayer, VerifyPlayerMoved,
};
use includes::*;
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

fn setup_test_app(test_queue: TestStepQueue) -> App {
    let mut app = App::new();
    app.insert_resource(test_queue)
        .init_resource::<UnfinishedSteps>()
        .add_systems(Update, (send_step_from_queue,).chain())
        .add_observer(handle_start_game)
        .add_observer(handle_quit_game)
        .add_observer(movement_test::handle_capture_player_position)
        .add_observer(movement_test::handle_move_player)
        .add_observer(movement_test::handle_verify_player_moved);
    app
}

fn run_tests(test_queue: TestStepQueue) {
    let app = setup_test_app(test_queue);
    bevy_barcamp::run(app);
}

fn send_step_from_queue(world: &mut World) {
    if world.resource::<UnfinishedSteps>().0 == 0 {
        if let Some(step) = world.resource_mut::<TestStepQueue>().steps.pop_front() {
            step.send(world);

            world.resource_mut::<UnfinishedSteps>().0 += 1;
            println!(
                "Sent step from queue. Steps remaining: {}",
                world.resource::<TestStepQueue>().steps.len()
            );
        } else {
            println!("All tests completed!");
            std::process::exit(0);
        }
    }
}

fn handle_start_game(
    _start_event: On<StartGameStep>,
    mut unfinished_steps: ResMut<UnfinishedSteps>,
) {
    println!("Handling StartGameStep");

    unfinished_steps.0 -= 1;

    println!("StartGameStep completed.");
}

fn handle_quit_game(_quit_event: On<QuitGameStep>, mut unfinished_steps: ResMut<UnfinishedSteps>) {
    println!("Handling QuitGameStep");

    unfinished_steps.0 -= 1;

    println!("QuitGameStep completed.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_processes_steps_sequentially() {
        // Setup
        let mut test_queue = TestStepQueue::default();
        test_queue.steps.push_back(step!(StartGameStep));
        test_queue.steps.push_back(step!(QuitGameStep));
        test_queue.steps.push_back(step!(StartGameStep));
        let mut app = setup_test_app(test_queue);

        // Verify initial state
        assert_eq!(app.world().resource::<TestStepQueue>().steps.len(), 3);
        assert_eq!(app.world().resource::<UnfinishedSteps>().0, 0);

        // Process first step
        app.update();
        assert_eq!(app.world().resource::<TestStepQueue>().steps.len(), 2);
        assert_eq!(app.world().resource::<UnfinishedSteps>().0, 0);

        // Process second step
        app.update();
        assert_eq!(app.world().resource::<TestStepQueue>().steps.len(), 1);
        assert_eq!(app.world().resource::<UnfinishedSteps>().0, 0);

        // Process third step
        app.update();
        assert_eq!(app.world().resource::<TestStepQueue>().steps.len(), 0);
        assert_eq!(app.world().resource::<UnfinishedSteps>().0, 0);
    }
}
