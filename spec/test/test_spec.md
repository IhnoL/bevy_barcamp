# General
- The tests are in their own src/bin directory as this is a rust convention

# Tests
- Each of the tests (e.g. movement) is it its own module and has a method "provide_steps()" that returns the TestEvents
- The TestEvents in the following are fetched and executed by the TestController

## Movement-Test 
- CapturePlayerPosition
- SendMoveEvent(MoveEvent(direction:=right))
- VerifyPlayerMoved(direction:=right)
- CapturePlayerPosition
- SendMoveEvent(MoveEvent(direction:=left))
- VerifyPlayerMoved(direction:=left)

## Jump-Test 
- TBD: Will be implemented later