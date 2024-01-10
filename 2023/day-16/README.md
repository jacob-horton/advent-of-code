Today we had to calculate the path of a light beam through a contraption full of mirrors and splitters. Mirrors would bounce the light in another direction, and splitters would split the beam into two, going opposite directions

# Part 1
Here, we had a fixed start position for the light and had to calculate the number of tiles that were energised (had a beam hit them). My solution involved storing a list of beams and simulating them at each time step. Each beam stored its position and direction.

At each step, the beam would check what tile it was on:
1. If it was empty, continue moving forward
2. If it was a mirror, change the beam's direction
3. If it was a splitter, change the direction of the current beam and create a new one in the opposite direction
4. If it was out of bounds, delete the beam

I also added an optimisation (and to prevent infinite loops), which was to keep track of all previous beams (i.e. all tiles that a beam had been on and the direction the beam was going). If there had already been a beam on that tile going the same direction, we don't need to recalculate it.


# Part 2
This was just to find the starting position that lit up the most tiles. The starting point could be any edge tile pointing inwards. This was just a simple iteration and check for the max value
