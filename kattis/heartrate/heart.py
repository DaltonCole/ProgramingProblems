
for _ in range(int(input())):
	b, p = list(map(float, input().split()))

	min_h = 60 * (b - 1) / p

	nor_h = 60.0 * b / p

	max_h = 60 * (b + 1) / p

	print('{:0.4f} {:0.4f} {:0.4f}'.format(min_h, nor_h, max_h))