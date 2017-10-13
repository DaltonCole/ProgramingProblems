from sys import stdin
from math import log

for line in stdin:
	number = int(line)

	if number == 0:
		break

	base = 2

	while True:
		exponent = log(number, base)
		if float(exponent).is_integer():
			print(int(exponent))
			break
		else:
			base += 1