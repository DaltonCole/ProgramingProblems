cases = int(input())

for i in range(cases):
	max_num = 0

	number = input()

	part = ''
	for j in number:
		part += j
		b = bin(int(part))
		max_num = max(max_num, b[2:].count('1'))

	print(max_num)