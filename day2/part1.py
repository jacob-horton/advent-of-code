horiz = 0
depth = 0

with open('inputs.txt', 'r') as f:
    line = f.readline()

    while line:
        split_line = line.split()
        direction = split_line[0]
        amount = int(split_line[1])
        if direction == 'forward':
            horiz += amount
        elif direction == 'down':
            depth += amount
        elif direction == 'up':
            depth -= amount

        line = f.readline()

print(horiz * depth)
