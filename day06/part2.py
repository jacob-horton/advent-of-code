#!/usr/bin/env python3
import copy

with open('input.txt', 'r') as f:
  ages = [int(x.strip()) for x in f.readline().split(',')]

num_per_age = [0]*9
for age in ages:
  num_per_age[age] += 1

# 256 days
for _ in range(256):
  new = num_per_age[0]

  for i in range(len(num_per_age)-1):
    num_per_age[i] = num_per_age[i+1]

  num_per_age[6] += new
  num_per_age[8] = new

print(sum(num_per_age))
