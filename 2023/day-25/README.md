For the final day, we had a system that was overloaded that we needed to snip 3 wires to split the massive circuit into 2 separate circuits

My intuition was that the wires that are the "busiest" would be the ones to snip. What I mean by busiest is for the shortest paths, of every possible pair of components, how many pass through that wire. If we have two almost separate circuits, any connection from one side to the other must go through one of the wires we need to snip. Since there are only 3, we know there aren't many options, so these 3 will have to be used lots when going from one side to the other

I'm not sure if this guarantees finding the 3 that need snipping, but it's at least a good approximation. If it is not a guarantee, I could do a bit of brute force, checking all the possible 3 wires that could be snipped, prioritising ones that are busier.

To check if there were two separate circuits, I did a flood fill from one node. If the number of nodes it can reach was less than the total number of nodes, then we have two separate circuits

