from typing import Tuple

MOD = 1000000007


def bad(left: str, right: str) -> bool:
    if (left == 'X' and right == 'O') or (left == 'O' and right == 'X'):
        return True
    return False


def recursion(s: str) -> Tuple[int, int, str]:
    '''
    Returns:
        Summed total
        Current num
        Right most non-F character
    '''
    if len(s) == 2:
        num = 1 if 'XO' == s or 'OX' == s else 0
        if s[1] != 'F':
            letter = s[1]
        elif s[0] != 'F':
            letter = s[0]
        else:
            letter = ''
        return num, num, letter
    else:
        # Left recursion
        total, num, right = recursion(s[:-1])
        if right != '':
            if bad(right, s[-1]):
                num = (num + 1) % MOD

        if s[-1] != 'F':
            right = s[-1]

        total = (total + num) % MOD
        return total, num, right


if __name__ == '__main__':
    cases = int(input())

    for i in range(cases):
        A = 0
        length = int(input())
        s = input()

        total = 0

        if length > 1:
            for n in range(length - 1):
                num, _, _ = recursion(s[n:])
                total = (total + num) % MOD

        print(f'Case #{i+1}: {total}')
