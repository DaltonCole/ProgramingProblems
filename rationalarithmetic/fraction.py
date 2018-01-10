from fractions import Fraction

cases = int(input())

for i in range(cases):
	x1, y1, op, x2, y2 = input().split()
	x1 = int(x1)
	y1 = int(y1)
	x2 = int(x2)
	y2 = int(y2)

	a = Fraction(x1, y1)
	b = Fraction(x2, y2)

	if op == '+':
		c = a + b
		print("{} / {}".format(c.numerator, c.denominator))
	elif op == '-':
		c = a - b
		print("{} / {}".format(c.numerator, c.denominator))
	elif op == '*':
		c = a * b
		print("{} / {}".format(c.numerator, c.denominator))
	elif op == '/':
		c = a / b
		print("{} / {}".format(c.numerator, c.denominator))