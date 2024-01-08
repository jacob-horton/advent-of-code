This was a really fun day for me. The problem was to find the position of horizontal and vertical mirrors given a top-down view of a reflected shape

E.g.
```
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
```

# Part 1
I used bitwise logic to solve this. For horizontal mirrors, each row was a number - `#` represented by a 1, and `.` represented by a 0. I looped through each possible position of the mirror. Then for each mirror position, I checked above and below simultaneously, and expanded out until I reached the edge (in which case, it was a valid mirror position), or a discrepancy (in which case, it was an invalid mirror position). Since these were just numbers, the comparisons were very fast

I repeated this for the vertical mirrors - using numbers generated from columns rather than rows. This is potentially a waste of compution, as it's just another representation of the same thing, but it simplifies the checking for mirrors

# Part 2
The catch here was that mirrors all had exactly 1 smudge, meaning that there would be a discrepancy of 1 tile between each side of the mirror. Using the bitwise logic, this was actually really easy to implement. I did an XOR between the two rows/columns that I was checking, then I counted the number of 1s. This would give me the number of differences, as if you XOR a number with itself, you get 0, and you get 1 otherwise.

Now I had the number of differences between two columns, if it was more than 0, then I could check if we'd already found a discrepancy. If not, continue to accept, otherwise, fail. I actually generalised this to work for any number of smudges. 

However, there is a slight bug in my code - it only checks the number of smudges in one number (which represents a row or column depending on which orientation of mirror it's checking). It doesn't check it between numbers. So if each column had one discrepancy, it would allow that. Luckily, the input provided didn't have that issue, so my code worked fine
