mod includes;
mod jump_test;
mod movement_test;
use bevy::prelude::*;
use includes::*;

#[derive(Default, Resource)]
pub struct TestQueue;

#[derive(Event)]
pub struct StartGameStep;

#[derive(Event)]
pub struct QuitGameStep;


pub fn queue_tests_on_startup(mut _controller: ResMut<TestController>) {
    let _registered_tests: &[fn() -> TestEvents] =
        &[movement_test::provide_steps, jump_test::provide_steps];
    let _tests: TestEvents = todo!("Collect test events from registered tests");
    todo!("Queue the gathered test events");
}


fn main() {
    let mut app = App::new();
    todo!("Run the configured Bevy app once the test runner wiring is ready");
}
