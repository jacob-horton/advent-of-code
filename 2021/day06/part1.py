#!/usr/bin/env python3
import copy

with open('input.txt', 'r') as f:
  ages = [int(x.strip()) for x in f.readline().split(',')]

# 80 days
for _ in range(80):
  new = 0
  for i, age in enumerate(ages):
    if age == 0:
      new += 1
      ages[i] = 6
    else:
      ages[i] -= 1

  ages.extend([8] * new)

print(len(ages))