cases = int(input())

for i in range(cases):
    length = int(input())
    s = input()

    # Remove Os
    s = s.replace('F', '')

    times = 0

    if len(s) > 0:
        current = s[0]
        for c in s:
            if c != current:
                times += 1
                current = c

    print(f'Case #{i+1}: {times}')
