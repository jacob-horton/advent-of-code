#!/usr/bin/env python3

def get_coord(line):
  coords = line.split(' -> ')
  coord1 = coords[0].split(',')
  coord2 = coords[1].split(',')

  return int(coord1[0]), int(coord1[1]), int(coord2[0]), int(coord2[1])

counts = {}

with open('input.txt', 'r') as f:
  line = f.readline()

  while line:
    line = line.strip()
    x1, y1, x2, y2 = get_coord(line)

    line = f.readline()
    if x1 != x2 and y1 != y2:
      continue

    min_x = min(x1, x2)
    max_x = max(x1, x2)
    min_y = min(y1, y2)
    max_y = max(y1, y2)
    
    for i in range(min_x, max_x+1):
      for j in range(min_y, max_y+1):
        coord = (i,j)
        if coord not in counts.keys():
          counts[coord] = 0
        counts[coord] += 1

count = 0
for value in counts.values():
  if value > 1:
    count += 1

print(count)