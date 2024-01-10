The puzzle for Day 22 was about bricks falling and colliding in a stack

These bricks are 3D cuboids

# Part 1
The task was to first let all the bricks settle, then work out which bricks could be removed without causing any others to fall i.e. any brick it supports needs to also be supported by another brick

I did this by creating a collision check function. Then, starting from the lowest brick, move it down until it collides with another brick or the floor, and then repeat with the next brick up.

After this, for each brick I made note of all the bricks it supports. Then, looping through each brick, if every brick that it supports is also supported by another brick, then it can be disintegrated

# Part 2
Here, we had to work out the number of bricks that would fall after breaking each brick. I did this by removing the brick, and using the settling function from part 1 to work out how many bricks fell as a result
