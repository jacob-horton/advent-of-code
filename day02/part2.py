horiz = 0
vert = 0
aim = 0

with open('inputs.txt', 'r') as f:
    line = f.readline()

    while line:
        split_line = line.split()
        direction = split_line[0]
        amount = int(split_line[1])
        if direction == 'forward':
            horiz += amount
            vert += aim * amount
        elif direction == 'down':
            aim += amount
        elif direction == 'up':
            aim -= amount

        line = f.readline()

print(horiz * vert)
