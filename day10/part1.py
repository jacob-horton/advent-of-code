pairs = '() [] {} <>'.split()

def is_opening(bracket):
  for pair in pairs:
    if pair[0] == bracket:
      return True

  return False

def get_opposite_bracket(bracket):
  for pair in pairs:
    if bracket in pair:
      return pair.replace(bracket, '')

def get_first_illegal_character(line):
  next_closing = []
  for char in line:
    if is_opening(char):
      next_closing.append(char)
    else:
      if next_closing.pop() != get_opposite_bracket(char):
        return char

points = {
  ')': 3,
  ']': 57,
  '}': 1197,
  '>': 25137
}

result = 0
with open('input.txt', 'r') as f:
  line = f.readline()

  while line:
    line = line.strip()
    illegal_char = get_first_illegal_character(line)

    if illegal_char != None:
      result += points[illegal_char]

    line = f.readline()

print(result)