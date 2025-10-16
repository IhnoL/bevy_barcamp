Based on the existing src/bin/test_runner/terrain_test.rs: Implement a terrain
▌ creation according to the specs that looks like [image 683x384 PNG] . Create only
▌ the minimal necessary code. Understand the architecture before starting. In the end
▌ the terrain_test should succeed and verify that the actual terrain was spawned e.g.
▌ by checking existence of one or multiple components. Implement only the minimal
▌ game logic!

▌ Create a minimum camera.                   

Add some global illumination to the camera module. I want to see the terrain.

Check the player in [image 683x384 PNG] and check the documents and diagrams in the
▌ spec directory (not test). I want you to create the player according to the spec.
▌ Also create a test in the style of the src/bin/test_runner/tests/terrain.rs terrain
▌ test which checks if the player actually exists in the world.

▌ Add the movement logic according to the specs. Make sure the existing src/bin/
▌ test_runner/tests/movement.rs succeeds in the end. Currently its only mocked, so it
▌ has to be implemented ;)

▌ Increase the MIN_MOVEMENT_DELTA to 50. Then add a wait step between the triggering
▌ of the movement event and the verify so that the player has some time to move. The
▌ wait step probably needs to be changed so that it does not sleep but instead waits
▌ a certain amount of update cycles to not block the thread.

When you check the [image 683x384 PNG] there is a mob: How can we add this mob to
▌ our scene as an image (the upper part) with multiple stick legs (like the player)
▌ for the lower part? Can you create an asset from the mob?

▌ Now create a test in the style of src/bin/test_runner/tests/player.rs which
▌ verifies the mob actually exists in the scene.

