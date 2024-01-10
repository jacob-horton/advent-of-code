For day 23, we were provided a fairly linear map of hiking trails. The only times there was a choice of different directions was at a slope (`^`, `>`, `v`, and `<`)

# Part 1
Here, we had to calculate the longest possible hike assuming we can only go down the slopes (i.e. only go in the direction they point). I did this by keeping track of the current position and direction, and looking at its neighbours for the next step. If any were a `.`, continue in that direction. If any were slope, check if we are able to go down it, and if so, add that to the frontier of paths to explore.

Once every path reached the end, return the one with the largest length

# Part 2
Now we can move up slopes, and we still want the longest path. This meant the number of paths was a lot greater. I approached this by finding just the intersection points, and the distances between each of them. Then I could perform a breadth first search, like part 1 and find the longest path

I found the intersections by finding `.`s with a slope next to them. Between each intersection, I counted the number of tiles as the length between the points
