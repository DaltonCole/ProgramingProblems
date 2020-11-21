import sys
import math

for i in sys.stdin:
	n = int(i)

	if n == 0 or n == 1:
		print(1)
		continue

	"""
	a = 0.5
	b = math.log(2 * math.pi)
	c = 2 * n
	d = math.log(n)
	e = 1 + (2 * n)
	f = math.log(10)
	"""

	# Kamenetsky formula
	answer = math.ceil((.5) * (math.log(2 * math.pi) - 2*n + (math.log(n) * (1 + (2 * n)))) / math.log(10))

	print(int(answer))