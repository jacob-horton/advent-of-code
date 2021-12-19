import math

cave = []
with open('input.txt', 'r') as f:
  line = f.readline()

  while line:
    line = line.strip()
    row = [int(x) for x in list(line)]
    cave.append(row)

    line = f.readline()

def get_neighbours(grid, current):
  neighbours = []
  if current[0] > 0:
    neighbours.append((current[0]-1, current[1]))
  if current[1] > 0:
    neighbours.append((current[0], current[1]-1))
  if current[0] < len(grid) - 1:
    neighbours.append((current[0]+1, current[1]))
  if current[1] < len(grid) - 1:
    neighbours.append((current[0], current[1]+1))

  return neighbours

def get_minimum(unvisited):
  min_val = math.inf
  min_node = None

  for key, node in unvisited.items():
    if node[0] < min_val:
      min_val = node[0]
      min_node = key

  return min_node

def dijkstra(grid):
  nodes = {}
  path_costs = {}

  for i in range(len(grid)):
    for j in range(len(grid[i])):
      nodes[(i,j)] = None
      path_costs[(i,j)] = math.inf

  frontier = [(0,0)]
  explored = []

  path_costs[(0,0)] = 0

  while len(frontier) != 0:
    current = frontier.pop()
    #explored.append(current)
    
    neighbours = get_neighbours(grid, current)
    for neighbour in neighbours:
      if neighbour in explored:
        continue

      prev_cost = path_costs[neighbour]
      new_cost = path_costs[current] + grid[neighbour[0]][neighbour[1]]

      if new_cost < prev_cost:
        path_costs[neighbour] = new_cost
        nodes[neighbour] = current

        frontier.append(neighbour)
    #explored.append(current)

  return nodes

def get_path_cost(grid, nodes, goal):
  current = goal
  cost = -grid[current[0]][current[1]]

  while current:
    cost += grid[current[0]][current[1]]
    current = nodes[current]

  return cost


print(get_path_cost(cave, dijkstra(cave), (len(cave) - 1, len(cave[0]) - 1)))
# 396 too high