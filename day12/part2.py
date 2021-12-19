import copy

cave = {}
with open('input.txt', 'r') as f:
  line = f.readline()

  while line:
    line = line.strip()
    parts = line.split('-')
    if parts[0] not in cave.keys():
      cave[parts[0]] = set()
    if parts[1] not in cave.keys():
      cave[parts[1]] = set()

    cave[parts[0]].add(parts[1])
    cave[parts[1]].add(parts[0])

    line = f.readline()

routes = []
def recursive(cave, explored=None, current='start', path=[], explored_small_twice=False):
  global routes
  path.append(current)
  
  if explored == None:
    explored = []
  
  if current == 'end':
    if path not in routes:
      routes.append(path)

    return

  if current in explored:
    return

  for route in cave[current]:
    explored_copy = copy.deepcopy(explored)
    if route != 'start' and current != 'start' and current[0].islower() and not explored_small_twice:
      recursive(cave, explored_copy, route, copy.deepcopy(path), True)

    if current[0].islower():
      explored_copy.append(current)
    recursive(cave, copy.deepcopy(explored_copy), route, copy.deepcopy(path), explored_small_twice)

recursive(cave)
print(len(routes))
# 17318 too low