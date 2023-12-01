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

def are_equivalent(num1, num2):
  return len(num1) == len(num2) and are_all_in(num1, num2)

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
        possible_numbers[i] = []
        # TODO: remove all other 9s
        for key, value in possible_numbers.items():
          possible_numbers[key] = [x for x in value if x != 9]
    if 2 in nums:
      print('2', nums)
      fits_in_any = False
      for inp in inputs:
        # Skip 8 and itself
        if len(inp) == 7 or inputs[i] == inp:
          continue
        if are_all_in(inp, inputs[i]):
          fits_in_any = True
          break

      if not fits_in_any:
        mapping[2] = i
        possible_numbers[i] = []
        for key, value in possible_numbers.items():
          possible_numbers[key] = [x for x in value if x != 2]

  for i in range(len(possible_numbers)):
    nums = possible_numbers[i]
    if 3 in nums:
      print(nums)
      if are_all_in(inputs[i], inputs[mapping[7]]):
        mapping[3] = i
        possible_numbers[i] = []
        for key, value in possible_numbers.items():
          possible_numbers[key] = [x for x in value if x != 3]

  for i in range(len(possible_numbers)):
    nums = possible_numbers[i]
    if 0 in nums:
      if are_all_in(inputs[i], inputs[mapping[1]]):
        mapping[0] = i
        possible_numbers[i] = []
        for key, value in possible_numbers.items():
          possible_numbers[key] = [x for x in value if x != 0]

  mapping[5] = get_num(possible_numbers, 5)
  mapping[6] = get_num(possible_numbers, 6)

  return mapping

def reverse_lookup(dictionary, value):
  key_list = list(dictionary.keys())
  val_list = list(dictionary.values())
  
  position = val_list.index(value)
  return key_list[position]

def get_index(all_numbers, num):
  for i, number in enumerate(all_numbers):
    if are_equivalent(number, num):
      return i

  return -1

total = 0
with open('input.txt', 'r') as f:
  line = f.readline()

  while line:
    line = line.strip()

    all_numbers = line.split(' | ')[0].split()
    wrong_digits = line.split(' | ')[1].split()

    mapping = get_mapping(all_numbers)
    digits = []

    for wrong_digit in wrong_digits:
      index = get_index(all_numbers, wrong_digit)
      digits.append(reverse_lookup(mapping, index))

    num = int(''.join([str(x) for x in digits]))
    print(digits, num)
    total += num

    line = f.readline()

print(total)
# 973046 too low
# 986360 too high