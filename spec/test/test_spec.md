# General
- The tests are in their own bin directory with a TestController steering the execution
- Between the tests the game is restarted by a Quit- and a following Start-Event
- The Test-Events are only executed in GameState::Running 

# Tests
- Each of the tests is it its own module and has a method that returns the TestEvents
- The TestEvents are fetched and executed by the TestController

## Movement-Test Events
- CapturePlayerPosition
- SendMoveEvent(MoveEvent(direction:=right))
- VerifyPlayerMoved(direction:=right)
- CapturePlayerPosition
- SendMoveEvent(MoveEvent(direction:=left))
- VerifyPlayerMoved(direction:=left)

## Jump-Test Events
- TBD: Will be implemented later