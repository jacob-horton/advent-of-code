The aim of day 3 was to find numbers ("part numbers") which were adjacent to symbols. The difficulty here is that the numbers aren't just single digits, and there many be multiple numbers adjacent to each symbol

# Part 1

Here, we had to sum up all the numbers that were adjacent to a symbol. I did this by looping through each line and when I found a digit, I'd add it to a buffer until I got to the end of the digits. Along the way, I also checked if any of the tiles surrounding the digit were a symbol, and if so, once I got to the end of the number, I added that to the sum

1. Loop over each line
2. Loop over each character in line
3. If it is a digit, add to a buffer and check if there is a symbol adjacent to it. If there is, keep track of that
4. Once reaching the end of the line or a non-digit, check if any of the digits were adjacent to a symbol and add to the sum if they were
5. Reset the symbol adjacent tracker and the buffer and continue

My algorithm isn't very efficient here as each digit checks all 8 tiles around it each time, even if this overlaps tiles checked by the previous digit. This wasn't a big concern though as performance was really good anyway

# Part 2

Here, we had to find numbers adjacent to gears ("\*"). We had to find gears that had exactly two adjacent numbers and multiply those

My solution was to first find all the gears ("\*"), then find the numbers adjacent to it, check there are exactly 2 and multiply those numbers

1. Find gears
2. Search 8 tiles around gears - going left to right, top to bottom. If there is a digit, we make note of its position. We continue until there is either a non-numeric character or when we move onto the next row. This gives us a list of positions of unique numbers (not positions of each digit, as then we would count numbers twice)
3. Only continue if there were 2 unique numbers around the gear
4. Get the number at each position - start with the position provided and search left and right until the character is no longer numeric. Then convert that substring to an integer
5. Multiply the integers and sum them all up
