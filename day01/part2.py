windows = []

# 1492
with open('inputs.txt', 'r') as f:
    line = f.readline()

    while line:
        num = int(line.strip())
        windows.append(0)
        
        for i in range(len(windows)-1, max(len(windows)-4, 0), -1):
            windows[i] += num

        line = f.readline()

count = 0
prev = None
for w in windows:
    if prev != None and w > prev:
        count += 1
    prev = w

print(count)
