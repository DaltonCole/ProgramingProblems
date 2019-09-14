cases = int(input())

for _ in range(cases):
	case, base, num = input().split()

	total = 0
	base = int(base)

	for i, n in enumerate(num):
		total += (int(n, base) ^ 2)

	print(total)