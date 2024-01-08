Today's problem was to put lenses in a box based on a hash value of their label

# Part 1
We started by just looping through each step of the initialisation sequence, and computing a hash of the step. The hash was simply adding the ascii code to an accumulator, multiplying that by 17, and getting the remainder after dividing by 256

# Part 2
In this part, the instructions actually had meaning. They came in the form:
1. `<label>=<number>` or
2. `<label>-`

The box each lens should be put in was just the hash (from part 1) of the label

For option 2, just remove that label from the box

For option 1, if there isn't a lens in the box with the same label, add it. Otherwise, overwrite its value, but keep the position

I used a jagged array for this. The outer array representing the boxes and the inner array representing a list of lenses, and the logic here is pretty self-explanatory
