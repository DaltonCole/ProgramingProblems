total = 0

cases = int(input())

for _ in range(cases):
	a, b = list(map(float, input().split()))

	total += (a * b)

print('{:0.3f}'.format(total))