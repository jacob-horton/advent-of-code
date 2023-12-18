Day 4's aim was to calculate winnings from scratch cards

# Part 1

For part 1, we just had to count how many of our numbers were in the winning numbers. Then the number of points was just $2^{(\text{matching numbers} - 1)}$

This was as simple as looping through the winning numbers, and seeing if my list of numbers contained that number. Count up the number of matches, and calculate the points from that

# Part 2

Instead of calculating points, the number of points actually represented the number of following scratchcards to get a copy of. So if you had 5 matches on card 10, you would win a copy of card 11, 12, 13, 14, and 15. And if you had 2 card 10s, then you would win two copies of each of those

I kept track of the number of copies of each card. As each win provides copies for following cards, we can just loop through in order. For each card, I calculated the number of matches on that card, then added to the copies of the following cards. The number of copies I added was the number of matches multiplied by the number of copies of the current card

1. Initialise number of copies of each card with 1 for each card (I did this with `or_insert(1)`)
2. Calculate the number of matches for the current card: $n$
3. For each of the copies of the following $n$ cards, add $n \times (\text{copies of current card})$ copies
4. Count up the number of scratchcards total
