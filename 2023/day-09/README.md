Day 9 was to predict the next number in a pattern. We were given a sequence of numbers. If we continuously found the difference between each pair of terms, we would eventually reach a difference of 0 between each term

# Part 1

This is essentially a derivative of the sequence. Once we found the derivative that was linear (i.e. all the numbers were equal), we could just propagate this up by adding that difference to the last term of the next derivative up. Once this reached the top, we had the next number in the sequence

# Part 2

This was very similar to part 1, just finding the number before the first one provided in the sequence. This involved subtracting the derivative each time instead of adding it, but it was essentially the same code otherwise
