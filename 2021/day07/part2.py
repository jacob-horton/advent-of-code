#!/usr/bin/env python3
import math

def fuel_for_distance(dist):
  return (dist * (dist+1)) // 2
  fuel = 0
  for i in range(dist):
    fuel += i + 1

  return fuel

def fuel_for_position(positions, dest):
  fuel = 0
  for position in positions:
    fuel += fuel_for_distance(abs(dest-position))

  return fuel

with open('input.txt', 'r') as f:
  positions = [int(x.strip()) for x in f.readline().split(',')]

min_pos = min(positions)
max_pos = max(positions)

best_fuel = math.inf
for i in range(min_pos, max_pos):
  fuel = fuel_for_position(positions, i)
  if fuel < best_fuel:
    best_fuel = fuel

print(best_fuel)