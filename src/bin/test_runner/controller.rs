use crate::movement_test::MovementTest;
use crate::test_types::TestQueue;

pub struct TestController {
    pub queue: TestQueue,
    pub movement_test: MovementTest,
}

impl TestController {
    pub fn run_tests(&mut self) {
        todo!("Run the registered tests");
    }
}
