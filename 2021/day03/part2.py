def most_frequent(lst):
    count_0 = 0
    count_1 = 0

    for num in lst:
        if num == 0:
            count_0 += 1
        elif num == 1:
            count_1 += 1

    if count_0 == count_1:
        return 1
    elif count_0 < count_1:
        return 1
    else:
        return 0

def invert(value):
    output = ""
    for c in value:
        if c == '0':
            output += '1'
        else:
            output += '0'

    return output

def get_report(lines):
    bits = []
    
    for line in lines:
        i = 0
        for c in line:
            if i >= len(bits):
                bits.append([])
            bits[i].append(int(c))
            i+=1


    output = ""
    for x in bits:
        output += str(most_frequent(x))

    return output, invert(output)

def rating(lines, index):
    common_input_bits = get_report(lines)[index]

    with open('input.txt') as f:
        lines = [x.strip() for x in f.readlines()]

    for i in range(len(common_input_bits)):
        new_lines = []
        for line in lines:
            if line[i] == common_input_bits[i]:
                new_lines.append(line)

        if len(new_lines) == 1:
            return new_lines[0]

        if len(new_lines) == 0:
            return lines

        lines = new_lines
        common_input_bits = get_report(lines)[index]

    return lines[0]


with open('input.txt', 'r') as f:
    lines = [x.strip() for x in f.readlines()]

most_common_input_bits, least_common_input_bits = get_report(lines)
ox = rating(lines, 0)
co2 = rating(lines, 1)
print(int(ox, 2) * int(co2, 2))
