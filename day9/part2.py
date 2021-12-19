from functools import reduce
import numpy as np

def convert_to_int_arr(arr):
  return [int(x) for x in list(arr)]

with open('input.txt', 'r') as f:
  grid = f.readlines()

def get_basin_size(grid, i, j, explored=None):
  if not explored:
    explored = set()

  if (i,j) in explored:
    return 0

  if grid[i,j] == 9:
    return 0

  size = 1
  explored.add((i,j))
  for x in range(max(0, i-1), min(i+2, len(grid))):
    if i == x: continue
    if grid[x, j] >= grid[i, j]:
      size += get_basin_size(grid, x, j, explored)

  for x in range(max(0, j-1), min(j+2, len(grid[i]))):
    if j == x: continue
    if grid[i, x] >= grid[i, j]:
      size += get_basin_size(grid, i, x, explored)

  return size

grid = np.array([convert_to_int_arr(x.strip()) for x in grid])

basins = []
for i, j in np.ndindex(np.shape(grid)):
  is_lowest = True
  for x in range(max(0, i-1), min(i+2, len(grid))):
    if x == i: continue
    if grid[i, j] >= grid[x, j]:
      is_lowest = False
      break

  if not is_lowest:
    continue

  for x in range(max(0, j-1), min(j+2, len(grid[i]))):
    if x == j: continue
    if grid[i, j] >= grid[i, x]:
      is_lowest = False
      break

  if is_lowest:
    basins.append(get_basin_size(grid, i, j))

top_three = list(sorted(basins, reverse=True))[:3]
print(top_three)
result = reduce(lambda x,y: x*y, top_three)
print('result:', result)
# 16664 too high
# 588 too high, but right for someone else
