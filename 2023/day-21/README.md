The puzzle today was to see what tiles an elf could get to after a certain number of steps. The elf could step up, down, left or right. And there were some obstacles (`#`) in the way.

# Part 1
This could be computed by keeping track of all the tiles that could be reached at step 1, then compute from those tiles, which ones could be reached and repeat

The numbers were small enough for this to be simulated

# Part 2
We find out the garden repeats infinitely, and the number of steps to take is much larger, so cannot be simulated

I didn't really like this part - it involved looking at the input or checking for a pattern in the outputs, which could not be applied to the example/test input, so it was not a very satisfying solution

When viewing the input, I noticed several things:
- The edges are all clear of obstacles
- Horizontally and vertically from the starting point is clear of obstacles
- The obstacles are very sparse
- There is a distinct diamond - a gap of obstacles

I also noticed that if there were no obstacles, the tiles filled after n steps would be a diamond with width/height of n, in a checkerboard pattern (alternating tiles). I called these "odd" or "even" tiles based on the `x` + `y` coordinates being odd or even

With all of this in mind, once the diamond of possible positions reaches the diamond of no obstacles, it is still an exact diamond (as the obstacles are sparse and there are a few blank lines to adjust any discrepancies)

I calculated the number of reachable tiles on an odd and even step in this diamond. Then I calculated the difference between the whole garden being filled and this diamond - giving the number of reachable steps in the corners (again for odd and even steps). When tiled, these 4 make a diamond with the centre at the corner of the original garden. As this repeated, I could calculate the number of big diamonds (from corners and from the centres), and sum up the number of reachable positions in those. The tricky part here was that alternating garden tiles are odd and even.

It happened that the number of steps was a good number, where the possible positions formed a diamond in the gap mentioned before. This is because it was a multiple of the size of the input + half the size of the input (or something to that effect) 

I saw other people just noticed a quadratic pattern at specific intervals that the number of steps happened to also lie on. This is essentially the same idea (but simpler), as the reason it is quadratic is that it forms a diamond at certain numbers of steps
