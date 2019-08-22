from math import sqrt

fib = [1, 1]

for i in range(10000):
	fib.append((fib[-1] + fib[-2]) % 1000000007)


cases = int(input())

for _ in range(cases):
	num = int(input())

	print(fib[num + 1])