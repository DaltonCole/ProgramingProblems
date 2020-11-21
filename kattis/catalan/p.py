d = [1, 1]

for i in range(2, 50001):
	d.append(int(d[-1] * (2 * ((2 * (i-1)) + 1)) // ((i - 1) + 2)))

cases = int(input())

for i in range(cases):
	num = int(input())
	print(d[num])