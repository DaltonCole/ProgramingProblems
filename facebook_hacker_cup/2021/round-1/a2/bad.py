from typing import Tuple

MOD = 1000000007


def bad(left: str, right: str) -> bool:
    if (left == 'X' and right == 'O') or (left == 'O' and right == 'X'):
        return True
    return False


A = 0

def recursion(s: str) -> Tuple[int, str]:
    '''
    Returns:
        Current Num
        Right most non-F character
    '''
    global A
    A += 1
    if len(s) == 2:
        num = 1 if 'XO' == s or 'OX' == s else 0
        if s[1] != 'F':
            letter = s[1]
        elif s[0] != 'F':
            letter = s[0]
        else:
            letter = ''
        return num, letter
    else:
        # Right recursion
        total, _ = recursion(s[1:])

        # Left recursion
        num, right = recursion(s[:-1])
        total = (total + num) % MOD
        if right != '':
            if bad(right, s[-1]):
                total = (total + 1) % MOD

        if s[-1] != 'F':
            right = s[-1]

        return total, right


if __name__ == '__main__':
    cases = int(input())

    for i in range(cases):
        A = 0
        length = int(input())
        s = input()

        adjacent = []

        total = 0

        if length > 1:
            total, _ = recursion(s)

        print(f'Case #{i+1}: {total}')
        print(A)
