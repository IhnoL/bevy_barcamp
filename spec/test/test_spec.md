# General
- The tests are in their own src/bin directory with a TestController steering the execution
- Between the tests the game is restarted by a Quit- and a following Start-Event

# Tests
- Each of the tests (e.g. movement) is it its own module and has a method "provide_tests()" that returns the TestEvents
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