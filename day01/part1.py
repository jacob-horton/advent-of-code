count = 0

prev = 0
with open('./inputs.txt', 'r') as f:
    line = f.readline()
    while line:
        num = int(line.strip())
        if num > prev:
            count += 1

        prev = num
        line = f.readline()

print(count)
