MOD = 1000000007


def bad(left: str, right: str) -> bool:
    if (left == 'X' and right == 'O') or (left == 'O' and right == 'X'):
        return True
    return False


def non_recursion(s: str) -> int:
    total = 0
    for i in range(len(s) - 1):
        for j in range(i + 1, len(s)):
            if i + 1 == j:
                num = 1 if 'XO' == (s[i] + s[j]) or 'OX' == (s[i] + s[j]) else 0
                if s[j] != 'F':
                    right = s[j]
                elif s[i] != 'F':
                    right = s[i]
                else:
                    right = ''
            else:
                if right != '':
                    if bad(right, s[j]):
                        num = (num + 1) % MOD
                if s[j] != 'F':
                    right = s[j]
            total = (total + num) % MOD
    return total


if __name__ == '__main__':
    cases = int(input())

    for i in range(cases):
        A = 0
        length = int(input())
        s = input()

        total = non_recursion(s)

        print(f'Case #{i+1}: {total}')
