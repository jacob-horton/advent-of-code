import numpy as np

numbers = []
boards = []

with open('input.txt', 'r') as f:
    lines = [x.strip() for x in f.readlines()]

numbers = [int(x) for x in lines[0].split(',')]

# Remove first two lines
del lines[0]
del lines[0]

board = []
for line in lines:
    if len(line.strip()) == 0:
        boards.append(np.array(board))
        board = []
    else:
        board.append([int(x) for x in line.split()])

boards.append(np.array(board))

def has_won(board, numbers):
    # Check rows
    for i in range(len(board)):
        diff = np.setdiff1d(board[i], numbers, assume_unique=True)
        if len(diff) == 0:
            return True

    # Check cols
    for i in range(len(board[0])):
        diff = np.setdiff1d(board[:,i], numbers, assume_unique=True)
        if len(diff) == 0:
            return True

    return False

def get_score(board, numbers):
    board_sum = 0

    diff = np.setdiff1d(board.flatten(), numbers, assume_unique=True)
    print(diff)
    for i in diff:
        board_sum += i

    return board_sum * numbers[-1]


partial_numbers = []

check_winning = False
for i in range(len(numbers)):
    partial_numbers.append(numbers[i])
    np_partial_numbers = np.array(partial_numbers)

    new_boards = []
    for board in boards:
        if not has_won(board, np_partial_numbers):
            new_boards.append(board)
        elif check_winning:
            print(boards[0])
            print(partial_numbers)
            print(get_score(boards[0], np_partial_numbers))

            #print(board)
            #print(partial_numbers)
            #print(get_score(board, np_partial_numbers))
            #exit(0)

    boards = new_boards
    if len(boards) == 1:
        check_winning = True