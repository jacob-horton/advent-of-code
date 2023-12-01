with open('input.txt', 'r') as f:
  polymer = f.readline().strip()
  rules = {}

  line = f.readline()
  while line:
    line = line.strip()
    if len(line) == 0:
      line = f.readline()
      continue

    parts = line.split(' -> ')
    rules[parts[0]] = parts[1]

    line = f.readline()

for i in range(10):
  new_polymer = ''
  for j in range(len(polymer) - 1):
    before = polymer[j]
    after = polymer[j+1]
    new_polymer += before + rules[before + after]
  polymer = new_polymer + after

def get_most_least_common(polymer):
  counts = {}

  for char in polymer:
    if char not in counts.keys():
      counts[char] = 0

    counts[char] += 1

  counts_sorted = sorted(counts.items(), key=lambda x:x[1])
  return counts_sorted[-1][1], counts_sorted[0][1]

most_common, least_common = get_most_least_common(polymer)
print(most_common - least_common)

# 1588 too low