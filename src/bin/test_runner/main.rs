mod events;
mod includes;
mod jump_test;
mod movement_test;
mod terrain_test;
use bevy::prelude::*;
use bevy_barcamp::game::state::GameState;
use events::{QuitGameStep, StartGameStep};
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
    test_queue.steps.extend(terrain_test::provide_steps());
    test_queue.steps.push_back(step!(QuitGameStep));

    test_queue.steps.push_back(step!(StartGameStep));
    test_queue.steps.extend(jump_test::provide_steps());
    test_queue.steps.push_back(step!(QuitGameStep));

    let base_app = bevy_barcamp::run(App::new());
    let mut app = setup_test_app(base_app, test_queue);
    app.run();
}

fn setup_test_app(app: App, test_queue: TestStepQueue) -> App {
    let mut app = app;
    app.insert_resource(test_queue)
        .init_resource::<UnfinishedSteps>()
        .add_systems(Update, (send_step_from_queue,).chain())
        .add_systems(OnEnter(GameState::Running), handle_start_game)
        .add_systems(OnEnter(GameState::Uninitialized), handle_quit_game)
        .add_observer(movement_test::handle_capture_player_position)
        .add_observer(movement_test::handle_move_player)
        .add_observer(movement_test::handle_verify_player_moved)
        .add_observer(terrain_test::handle_verify_terrain_spawned);
    app
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

fn handle_start_game(mut unfinished_steps: ResMut<UnfinishedSteps>) {

    println!("Handling StartGameStep");
    unfinished_steps.complete_step();
    println!("StartGameStep completed.");
}

fn handle_quit_game(mut unfinished_steps: ResMut<UnfinishedSteps>) {

    println!("Handling QuitGameStep");
    unfinished_steps.complete_step();
    println!("QuitGameStep completed.");
}
