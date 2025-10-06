mod events;
mod includes;
mod jump_test;
mod movement_test;
use bevy::prelude::*;
use includes::*;
use events::{QuitGameStep, StartGameStep};

#[derive(Default)]
pub struct TestQueue {
    pub steps: Vec<Box<dyn TestStep>>,
}

fn main() {
   // Use the app created in "lib.rs"
    // add quit and start tep to queue
    // fetch test steps from movement_test and add to queue
    // another quit and start step
    // fetch jump test
    // run the tests

}
