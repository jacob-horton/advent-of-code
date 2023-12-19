Today's challenge was to work out whether a part is accepted or rejected based on conditions on a few properties. The properties were numbers for `x`, `m`, `a`, and `s`, and the conditions were just `<` or `>` some number

There were workflows - a set of rules which would describe which rule to apply next to a given part if it met a specific condition. Some rules would instead specify if the part should be accepted or rejected

# Part 1

The tricky part here was parsing the input. I did this with nom. Then once I had the workflows parsed, I started with the `in` workflow, and find the first rule that matched. Then apply the workflow specified by the matched rule. If the workflow was accept (`A`) or reject (`R`), I stopped at that point and returned `true` if it was accepted or false otherwise

# Part 2

Part 2 was a lot harder - we had to find the number of possible combinations of `x`, `m`, `a`, and `s` that would be accepted. This could not be done by brute force as there were 4000 options for each of those properties, totalling `256,000,000,000,000` combinations. Instead, I recursively calculated the range of values that would result in acceptance. I just had to search each possible path at each condition - meeting it and not meeting it

1. Start with `in` workflow
2. Loop through each rule
3. If there was no condition, move on to the next workflow with the existing ranges
4. For the case of matching the condition - recursively search the matched workflow using the range of values that match the condition
5. After this, calculate the new range of properties that didn't meet the condition and continue to the next rule
6. If we accept the part, return the number of combinations - the product of the range lengths of each property
7. If we reject the part, return 0
8. Sum all this up to get the total combinations

I wondered if we needed to account for overlapping ranges, but this was not needed. I believe this is because there is only 1 way to classify each part, so there are never multiple paths for the same combination

I also had issue with several off-by-one errors. The comparisons were strictly less than and strictly greater than. My ranges included the start, but not the end. This meant:

- The end of the ranges I began with needed to be 4001, as the max number was 4000, and my range end was exclusive
- The range max, in the less than comparison, was reduced to the number in the condition. This is because the max is exclusive and it was strictly less than, so these work nicely together
- The range min, in the greater than comparison, was increased to the number in the condition + 1. This is because it is strictly greater than and the min is inclusive, so I needed to go one above
