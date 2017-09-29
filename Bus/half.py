cases = int(input())

for i in range(cases):
	a = int(input())

	total = 1

	for i in range(1, a):
		total = (total * 2) + 1

	print(total)