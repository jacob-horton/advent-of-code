dots = []
x_folds = []
y_folds = []

dimensions = []

reading_dots = True
with open('input.txt', 'r') as f:
  line = f.readline()

  while line:
    line = line.strip()
    if len(line) == 0:
      reading_dots = False
      line = f.readline()
      continue

    if reading_dots:
      parts = [int(x) for x in line.split(',')]
      dots.append([parts[0], parts[1]])
    else:
      important = line.split()[-1]
      x_or_y = important.split('=')[0]
      num = int(important.split('=')[1])

      if x_or_y == 'x':
        x_folds.append(num)
      else:
        y_folds.append(num)

    line = f.readline()

max_x = max([x[0] for x in dots])
max_y = max([x[1] for x in dots])
dimensions = [max_x, max_y]

print(dimensions)

for fold in x_folds:
  dimensions[0] = max(fold, dimensions[0] - fold)
  for dot in dots:
    if dot[0] > fold:
      dot[0] = 2*fold - dot[0]


for fold in y_folds:
  dimensions[1] = max(fold, dimensions[1] - fold)
  for dot in dots:
    if dot[1] > fold:
      dot[1] = 2*fold - dot[1]


for j in range(dimensions[1] + 1):
  line = ''
  for i in range(dimensions[0] + 1):
    if [i, j] in dots:
      line += '#'
    else:
      line += ' '
  print(line)
