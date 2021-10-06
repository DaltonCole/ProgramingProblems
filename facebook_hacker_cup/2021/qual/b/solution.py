
def print_board(board):
    for row in board:
        print(row)


def single_case(board):
    used_squares = set()
    # Count row solutions
    for i, row in enumerate(board):
        if 'O' not in row and row.count('.') == 1:
            used_squares.add((i, row.find('.')))
    # Count number of col solutions
    for i in range(n):
        col = [row[i] for row in board]
        if 'O' not in col and col.count('.') == 1:
            used_squares.add((col.index('.'), i))

    return [1 for _ in used_squares]


if __name__ == '__main__':
    cases = int(input())

    for case in range(cases):
        n = int(input())

        board = []

        for _ in range(n):
            board.append(input())

        solutions = []
        # Count row solutions
        for row in board:
            if 'O' not in row:
                solutions.append(row.count('.'))
        # Count number of col solutions
        for i in range(n):
            col = [row[i] for row in board]
            if 'O' not in col:
                solutions.append(col.count('.'))

        solutions.sort()

        if len(solutions) == 0:
            print(f'Case #{case + 1}: Impossible')
        else:
            answer = solutions[0]

            if answer == 1:
                solutions = single_case(board)

            count = solutions.count(answer)
            print(f'Case #{case + 1}: {answer} {count}')

        # print_board(board)
