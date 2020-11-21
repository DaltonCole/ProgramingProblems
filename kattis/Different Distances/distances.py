while True:
	a = input()

	if a == '0':
		break

	x1, y1, x2, y2, p = a.split()
	x1 = float(x1)
	x2 = float(x2)
	y1 = float(y1)
	y2 = float(y2)
	p = float(p)

	output = pow(pow(abs(x1 - x2), p) + pow(abs(y1 - y2), p), 1/p)

	print(output)