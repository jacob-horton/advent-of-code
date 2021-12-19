numbers = [
  list('abcefg'),
  list('cf'),
  list('acdeg'),
  list('acdfg'),
  list('bcdf'),
  list('abdfg'),
  list('abdefg'),
  list('acf'),
  list('abcdefg'),
  list('abcdfg')
]

def get_num(possible_nums, num):
  num_pos = list(possible_nums.values()).index([num])
  return list(possible_nums.keys())[num_pos]

def are_all_in(lst, vals):
  for val in vals:
    if not val in lst:
      return False
    
  return True

def get_mapping(inputs):
  possible_mappings = {}

  for i in list('abcdefg'):
    possible_mappings[i] = list('abcdefg')

  possible_numbers = {}
  for i, inp in enumerate(inputs):
    possible_numbers[i] = []
    for j, num in enumerate(numbers):
      if len(num) == len(inp):
        possible_numbers[i].append(j)


  mapping = {
    1: get_num(possible_numbers, 1),
    4: get_num(possible_numbers, 4),
    7: get_num(possible_numbers, 7),
    8: get_num(possible_numbers, 8)
  }

  return mapping

with open('input.txt', 'r') as f:
  line = f.readline()

  while line:
    line = line.strip()

    all_numbers = line.split(' | ')[0]
    digits = line.split(' | ')[1]
    mapping = get_mapping(line)
    
     

    line = f.readline()