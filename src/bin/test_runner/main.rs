mod events;
mod includes;
mod tests;
use bevy::prelude::*;
use bevy_barcamp::game::state::GameState;
use events::{QuitGameStep, StartGameStep};
use includes::*;
use macros::step;
use tests::{terrain, TestsPlugin};
use std::collections::VecDeque;

#[derive(Default, Resource)]
pub struct TestStepQueue {
    pub steps: VecDeque<Box<dyn TestStep>>,
}

fn main() {
    let mut test_queue = TestStepQueue::default();

    /* test_queue.steps.push_back(step!(StartGameStep));
    test_queue
        .steps
        .extend(tests::movement::provide_steps());
    test_queue.steps.push_back(step!(QuitGameStep));*/

    test_queue.steps.push_back(step!(StartGameStep));
    test_queue.steps.extend(terrain::provide_steps());
    test_queue.steps.push_back(step!(QuitGameStep));

    /*  test_queue.steps.push_back(step!(StartGameStep));
    test_queue.steps.extend(tests::jump::provide_steps());
    test_queue.steps.push_back(step!(QuitGameStep));*/

    let base_app = bevy_barcamp::run(App::new());
    let mut app = setup_test_app(base_app, test_queue);
    app.run();
}

fn setup_test_app(app: App, test_queue: TestStepQueue) -> App {
    let mut app = app;
    app.insert_resource(test_queue)
        .init_resource::<UnfinishedSteps>()
        .add_systems(Update, send_step_from_queue)
        .add_systems(OnEnter(GameState::Running), handle_start_game)
        .add_systems(OnEnter(GameState::Uninitialized), handle_quit_game)
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
