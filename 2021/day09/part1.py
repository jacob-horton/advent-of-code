import numpy as np

def convert_to_int_arr(arr):
  return [int(x) for x in list(arr)]

with open('input.txt', 'r') as f:
  grid = f.readlines()

grid = np.array([convert_to_int_arr(x.strip()) for x in grid])

result = 0
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
    result += grid[i,j] + 1

print(result)
# 16664 too high
# 588 too high, but right for someone else