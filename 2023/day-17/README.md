Today, elves had to carry crucibles through a grid that has different heat losses at each block. The goal was to find the path between the top left and bottom right with the least heat lost

# Part 1
The elves could move at most 3 blocks in a straight line.

This was essentially a tweaked A\* algorithm - using manhatten distance as the heuristic. At each point, I added to the frontier the possible tiles the elf could reach by going in a single direction. This meant going -2, -1, 1, or 2 tiles in the direction. Then for each of those points, make the direction perpendicular and repeat. At each iteration, I explored the current position in the frontier with the smallest heat loss + heuristic (this is the A\* part)

# Part 2
Here, the elves could move a minimum of 4 block and maximum of 10. I did the same as part 1, but instead of exploring -2, -1, 1 and 2 tiles in each direction, explore -10 ... -4, 4 ... 10
