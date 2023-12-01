import copy

cave = {}
with open('test_input2.txt', 'r') as f:
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
def recursive(cave, explored=None, current='start', path=[]):
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

  if current[0].islower():
    explored.append(current)

  for route in cave[current]:
    recursive(cave, copy.deepcopy(explored), route, copy.deepcopy(path))
    
recursive(cave)
print(len(routes))