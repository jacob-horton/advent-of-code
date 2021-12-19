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

  for i in range(len(possible_numbers)):
    nums = possible_numbers[i]
    if 9 in nums:
      if are_all_in(inputs[i], inputs[mapping[4]]):
        mapping[9] = i
        # TODO: remove all other 9s
        for key, value in possible_numbers.items():
          possible_numbers[key] = [x for x in value if x != 9]
    if 3 in nums:
      if are_all_in(inputs[i], inputs[mapping[7]]):
        mapping[3] = i
        for key, value in possible_numbers.items():
          possible_numbers[key] = [x for x in value if x != 3]

  for i in range(len(possible_numbers)):
    nums = possible_numbers[i]
    if 0 in nums:
      if are_all_in(inputs[i], inputs[mapping[1]]):
        mapping[0] = i
        for key, value in possible_numbers.items():
          possible_numbers[key] = [x for x in value if x != 0]

  return mapping


  np.count_nonzero(arr==val)

# 2 5 6

get_mapping('be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb'.split())
# Know 1, 8, 4, 7
# 1 in 0, 3
# 4 in 9
# 7 in 3