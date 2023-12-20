Today's challenge was to find the distance a boat travelled. We had a set time for the race, and we could hold a button to charge up the speed for as long as we wanted. The number of milliseconds you hold the button for is the distance the boat will travel each millisecond after you let go.

We had to find the number of different ways there are to beat a given distance for the race (each way being counted as holding the button for a different length of time)

# Part 1

Here, I implemented a brute force algorithm as the numbers were small. I calculated the distance travelled as $(T - t) * t$ where $T$ is the total time of the race and $t$ is the time holding the button. This is because $t$ is the velocity the boat will be travelling at, and the time it will travel at that velocity is $(T - t)$ (the time remaining after holding the button). Then distance is speed multiplied by time

Going through each time $t$ from $0$ to $T$, I counted up the number that exceeded the distance provided

# Part 2

Instead of 3 separate numbers for 3 races, it was 1 race with 1 big number for time and 1 for distance. My time was 53,717,880, which actually ran with the same code from part 1. But I knew I could do better.

My idea was to find the first value that beat the given distance. Since the distance travelled is a quadratic of $t$, it is symmetrical. I could use this to work out the last value as well. Then it was as simple as doing end - start (with an adjustment for an off-by-one error)

E.g.

```
Time = 8ms
Distance = 10

\# = Not further
@ = Further
number = Time holding

012345678
##@@@@@##
```

$\text{first further} = 2$

$\text{last further} = (8 - \text{first further}) = 6$

$(6 - 2) + 1 = 5$

---

The interesting part here was finding the first value. I initially did a linear search. I improved this to use a binary search. we know the point it beats the distance is in the first half and we know all points between that and half way will also beat the distance. This made it perfect for a binary search - searching a lower value if we're beating the distance, and searching higher if we're losing.

While revisiting this, I realised I could just use the quadratic formula to find the roots of this equation algebraically. I could take the lower one and use that with the algorithm above. This ended up not being any faster than the binary search (sometimes slower). I believe this is because the square root algorithm is pretty slow

**Here's the maths:**

$T$ = total time

$t$ = time holding

$d$ = distance

We want to find $(T - t) * t > d$

When we rearrange this and plug it into the quadratic formula, we get:

Roots are at $t = \dfrac{T \pm \sqrt{T^{2} - 4d}}{2}$

I want the lower root (when it first beats the distance), so took

$t = \dfrac{T - \sqrt{T^{2} - 4d}}{2}$

Then I added one to correct for an off-by-one error
