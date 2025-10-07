# General
- The tests are in their own src/bin directory as this is a rust convention

# Tests
- Each of the tests (e.g. movement) is it its own module and has a method "provide_steps()" that returns the test-steps 
- The MovementStepHandler and the test-steps declared are also located in this module 
- An "includes" module declares the generally used dependencies for the tests

## Movement-Test 
- CapturePlayerPosition
- MovePlayer(direction:=right)
- VerifyPlayerMoved(direction:=right)
- CapturePlayerPosition
- MovePlayer(direction:=left)
- VerifyPlayerMoved(direction:=left)

## Terrain-Test
- VerifyTerrainSpawned()

## Jump-Test 
- TBD: Will be implemented later