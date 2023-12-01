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

def get_closing_sequence(line):
  next_closing = []
  for char in line:
    if is_opening(char):
      next_closing.append(char)
    else:
      next_closing.pop()

  return ''.join(reversed([get_opposite_bracket(x) for x in next_closing]))

def get_score(closing_sequence):
  score = 0
  for char in closing_sequence:
    score *= 5
    score += points[char]

  return score

points = {
  ')': 1,
  ']': 2,
  '}': 3,
  '>': 4
}

result = 0
with open('input.txt', 'r') as f:
  line = f.readline()

  scores = []
  while line:
    line = line.strip()
    closing_sequence = get_closing_sequence(line)
    scores.append(get_score(closing_sequence))

    line = f.readline()

  print(sorted(scores)[len(scores)//2+1])

# 2378410308 too low
# 2429644557