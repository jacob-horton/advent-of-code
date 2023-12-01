def should_count(num):
  return len(num) == 2 or len(num) == 4 or len(num) == 3 or len(num) == 7

count = 0
with open('input.txt', 'r') as f:
  line = f.readline()

  while line:
    line = line.strip()

    digits = line.split(' | ')[1].split()
    to_be_counted = filter(should_count, digits)
    count += len(list(to_be_counted))

    line = f.readline()

print(count)