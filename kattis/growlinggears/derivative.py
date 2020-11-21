cases = int(input())

for i in range(cases):
	count = int(input())


	best_number = 0
	max_height = -999999999999999

	for j in range(count):
		a, b, c = input().split()

		a = int(a)
		b = int(b)
		c = int(c)

		# Derivative
		# T' = -2aR + b
		# Max
		# R = b / (2a)
		max_r = b / (2 * a)

		# T = -a*R^2 + b*R + c
		y =  (-1 * a * pow(max_r, 2)) + (b * max_r) + c

		if y > max_height:
			max_height = y
			best_number = j + 1

	print(best_number)