cases = int(input())

for _ in range(cases):
        case, base, num = map(int, input().split())

        total = 0

        num = str(int(str(num), base))

        for i, n in enumerate(num):
                total += (int(n) ^ 2)

        print(total)
