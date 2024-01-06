Today's puzzle was to navigate a map using L/R directions, and a map that described what node is next if you go left/right from a specific point

# Part 1

The input of L/R instructions repeated infinitely. The aim was to go from the node AAA to ZZZ. I solved this by keeping track of the current node and instruction, working out the next node based on this then repeating until reaching ZZZ

# Part 2

Instead of 1 path, we had to look at all nodes ending in A for starting positions and all ending in Z for end positions. We had to traverse from each starting point simultaneously and work out when you reach all the nodes ending in Z at the same time.

This number was way too big to compute by brute force. The trick here was to find how long it took for each path to reach Z. I noticed that after reaching Z, the next step was always back to the original start node, and the instructions were back to 0. This meant it was a perfect loop, so the time each Z was reached was just a multiple of the number of steps time to first reach that Z

This meant that I could find the lowest common multiple between all the loop lengths
