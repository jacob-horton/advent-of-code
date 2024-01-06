Today's challenge was to find the rank of a bunch of hands, in a game similar to poker.

# Part 1

First, I worked out the hand type of the given 5 cards. This could be 5 of a kind, 4 of a kind, etc. and there was a fixed ranking for which hand types were better than others. The way I worked the hand type out was by counting the number of each card in the hand, then finding the biggest number of duplicate cards. There was some extra checking required for full house, three of a kind, two pair, and one pair as these didn't just look at the largest number of duplicate cards, but also the remaining cards.

To compare two hands, you first compare the hand type. If they have the same type, then you have to compare the individual cards, from left to right. The first one to have a stronger card is a stronger overall hand. The card strengths were fixed

# Part 2

Here, the `J` cards became jokers. They now have the weakest individual card strength, but they can pretend to be another card to get the best hand type. To calculate the hand type now, I just had to add the number of jokers to the highest card count.

I also had to make sure that I didn't count the jokers twice (e.g. in a full house) - I only counted up the number of each non-joker card
