Day 14 had a reflector dish with round and cube rocks on it. The round rocks could roll, but the cube ones could not. You could tilt the dish in any of the 4 compass directions and all the round rocks would roll until colliding with something

# Part 1
The goal was just to tilt the lever so all the rocks slide to the north side. I did this by looping through each row (from top to bottom) and moving the rocks up until they hit something. This worked specifically because I started at the top. This is because at each row, we know all everything above it has already been rolled as far as possible.

# Part 2
Here, we had to tilt the lever north, then west, then south, then east - each being 1 cycle. Then repeat this 1000000000 times.

I repeated the logic from part 1, but with different directions. I had to start looping from a different point to ensure the rocks piled up correctly for each direction. I repeated this and recorded the state after each cycle until it started repeating, at which point, I knew it would repeat until the end
