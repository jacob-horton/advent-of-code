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

def add_pair(pairs, pair, count=1):
  if pair not in pairs.keys():
    pairs[pair] = 0

  pairs[pair] += count

  return pairs

last_letter = polymer[-1]
pairs = {}
# Add starting pairs
for j in range(len(polymer) - 1):
  key = polymer[j] + polymer[j + 1]
  pairs = add_pair(pairs, key)

for i in range(40):
  next_pairs = {}
  for pair, count in pairs.items():
    new = rules[pair]
    next_pairs = add_pair(next_pairs, pair[0] + new, count)
    next_pairs = add_pair(next_pairs, new + pair[1], count)

  pairs = next_pairs


def get_most_least_common(pairs):
  counts = {}

  # Only add the first letter from each pair, as all except the last letter are accounted for twice
  for pair, count in pairs.items():
    counts = add_pair(counts, pair[0], count)
  
  # Add the last letter to the counts
  counts[last_letter] += 1

  counts_sorted = sorted(counts.items(), key=lambda x:x[1])
  return counts_sorted[-1][1], counts_sorted[0][1]

most_common, least_common = get_most_least_common(pairs)
print(most_common - least_common) # IDK why subtract one