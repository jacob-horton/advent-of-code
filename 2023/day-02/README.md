The game here involves an elf pulling out handfuls of cubes from a bag. They can be red, green, or blue. The input is the number of each colour cube for each handful

# Part 1

Here, we had to work out which games were possible given that each bag only contained 12 red cubes, 13 green cubes and 14 blue cubes. All we had to do here was find the games where every individual handful had no more than any of possible amounts of each cube colour i.e. games where none of the individual handfuls had more than 12 red cubes, 13 green cubes or 14 blue cubes.

The fun here was using the [nom](https://docs.rs/nom/latest/nom/) crate to parse the input. I parsed the games into a list of "subset" (handful) structs, then looped through each ones and summed the IDs of any game that met the criteria

# Part 2

Instead of having a set maximum number of each colour cube, we needed to work out the minimum number of each colour cube would make each game work. I made a `piecewise_max` method on the "subset" (handful) structs, which made it really easy to get the maximum of each individual colour in a given game
