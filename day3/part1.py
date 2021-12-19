def most_frequent(lst):
    count_0 = 0
    count_1 = 0

    for num in lst:
        if num == 0:
            count_0 += 1
        elif num == 1:
            count_1 += 1

    if count_0 == count_1:
        return -1
    elif count_0 < count_1:
        return 1
    else:
        return 0

with open('input.txt') as f:
    line = f.readline().strip()
    bits = []
    
    while line:
        i = 0
        for c in line:
            #print(c, i)
            if i >= len(bits):
                bits.append([])
            bits[i].append(int(c))
            i+=1

        #print(bits)
        line = f.readline().strip()

    output = ""
    for x in bits:
        #print(x)
        output += str(most_frequent(x))
    print(output)
