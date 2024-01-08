The goal of day 12 was to find out the number of different arrangements given the position of damaged, operational and unknown hot springs, as well as the run lengths of damaged hot springs

E.g. `???.### 1,1,3` means runs of damaged hot springs of length 1, 1 and 3 (in that order, always a gap between). The `?`s are the unknown springs and `#` are damaged

# Part 1
Here, we had to find the number of arrangements of each line. I did this by recursively filling in a damaged or operational spring for each unknown. And then count up valid solutions

# Part 2
The input was tripled in size, and a `?` was put between the repeated sections, meaning we couldn't just use the answers from part 1 and do something clever with them

Instead, I tried multiple methods. I tried working from the run lengths and loop through every possible arrangement and match it against the known/unknown springs to see if it was possible. This was nowhere near fast enough though

I also tried optimising my part 1 code, but didn't get very far. I didn't think to cache as I don't think my solution lended itself to much caching, but I may have been wrong

I came across [this writeup](https://advent-of-code.xavd.id/writeups/2023/day/12/) which helped a lot. I skimmed through the explanation part, avoiding the code to test my understanding. The solution was similar, but we could eliminate some recursive calls by checking at each stage if it was still possible to match the run lengths, if not, skip. Also, if we matched the length of a run, then we could make sure the next spring was operational

The last part to the puzzle was caching, which helped a lot


