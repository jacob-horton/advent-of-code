Today, we had a bunch of directions and distances that described the perimeter of a lagoon. The goal was to find the area inside i.e. the number of tiles to dig for the lagoon

# Part 1
The input here was relatively small. I just traced out the perimeter and did a flood fill

# Part 2
Instead, the input was much larger - a flood fill was not reasonable here. Instead, I stored all the corner positions, and did a calculation to find the area inside that shape. This used [Gauss's shoelace formula](https://en.m.wikipedia.org/wiki/Shoelace_formula) for the area. However, that would find the area on a continuous plane, but we have a discrete one, where the edges have width

Therefore, we need to account for the area missed, which is the area outside the path traced by taking the top left of each block

For example, in the diagram below, the blue represents the tiles, the black represents the outline of the area calculated, and the red is the missed area:
![Diagram]("diagram.png")
