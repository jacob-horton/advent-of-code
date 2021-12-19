import numpy as np

grid = []
with open('input.txt', 'r') as f:
  line = f.readline()

  while line:
    line = line.strip()
    grid.append([int(x) for x in list(line)])

    line = f.readline()

grid = np.array(grid)
flashes = 0

def flash(grid):
  global flashes
  for i in range(len(grid)):
    for j in range(len(grid[i])):
      if grid[i][j] > 9:
        flashes += 1
        grid[i][j] = -1 # Set to -1 so when 1 added, it goes to 0 

        min_x = max(0, i-1)
        max_x = min(len(grid), i+2)
        min_y = max(0, j-1)
        max_y = min(len(grid[i]), j+2)
        for x in range(min_x, max_x):
          for y in range(min_y, max_y):
            if grid[x,y] != 0:
              grid[x,y] += 1

        grid = flash(grid)

  return grid

def iterate(grid):
  for i in range(len(grid)):
    for j in range(len(grid[i])):
      grid[i][j] += 1

  flash(grid)