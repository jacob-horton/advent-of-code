The problem for day 10 was to work out properties of a pipe loop

# Part 1

Given an input of straight pipes, corner pieces and a start pipe, we had to work out the number of steps to the farthest point in the loop. I solved this by following the pipe round in each direction coming out of the starting position. Once they met, I knew the number of steps to the furthers point in the loop - just the number of iterations in the loop to get to this point

# Part 2

This was a very interesting part. We had to find all the empty tiles that were enclosed by the loop.

## Initial Idea - Flood Fill

This initial idea worked, but I ended up deleting the code because it felt a bit hacky. I still think it's an interesting idea though, so here's what I did

The idea was to flood fill, but the issue was that there could be two pipes running parallel with no air gap between them. This would not be accounted for in the flood fill algorithm, but is required to reach other areas enclosed by the pipes

I solved this by "expanding" the grid to add more air gaps. I did this by multiplying the coordinates of each corner by 2, the reconnecting the corners. I could flood fill this, and count up the number of tiles that have both x and y coordinates divisible by 2 (as these would be from the original grid)

E.g.

```
.F7.
.||.
.||.
.LJ.
```

Became

```
..F-7..
..|.|..
..|.|..
..|.|..
..|.|..
..L-J..
```

You can see an air gap opened up, allowing for flood fill to do it's magic. But these new air gaps did not have even x coordinates, so they were not included in the sum of tiles (as they do not lie on the original grid)

## Revised Idea - Ray Casting

I had remembered that I could find if a point was inside a shape by counting the number of intersections between a ray going from the outside to that point, and the shape edges. If this is even, the point is outside the shape, and if it is odd, it is inside. I ran this algorithm for each cell that was not part of the pipe loop, and counted up the ones inside

This ended up not being any faster than flood fill. I'm not sure if that's an implementation issue or if it is just a similar speed algorithm, but this was a bit disappointing

I also tried to combine the approaches - do the ray cast. Once I found a tile inside, do a flood fill, and skip checking the tiles it had filled. This ended up not saving much time and just cluttering the code. The reason why is because most regions were very small - often only 1 tile, so the flood fill did nothing here
