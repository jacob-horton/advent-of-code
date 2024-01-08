For day 11 we had a grid of mostly empty space, with some galaxies in it. We had to expand the space between the stars. Then we had to count up the shortest paths between each pair of galaxies

# Part 1
Here, we just had to find every row or column that was fully empty, then duplicate it. Then we could calculate the distances between all the pairs

# Part 2
This wasn't much harder than part 1, we just had to add lots more space between the stars. This meant that we could just duplicate the rows/columns in memory, instead we just had to calculate the distance. I did this by looping through each pair of galaxies and then counting the number of empty columns and rows between the two. Multiply these numbers by the desired amount, then add them up along with any rows or columns that were not empty

