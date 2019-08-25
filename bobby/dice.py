from math import factorial

# Formula:
# https://www.varsitytutors.com/hotmath/hotmath_help/topics/binomial-probability

def combination(n, x):
	return (factorial(n)) / (factorial(n - x) * factorial(x))

def binomial_probability(p, n, x):
	return combination(n, x) * pow(p, x) * pow(1 - p, n - x)


cases = int(input())

for _ in range(cases):
	r, s, x, y, w = list(map(int, input().split()))

	proability_of_success = (s - r + 1) / s

	total = 0

	for x_sucesses in range(x, y+1):
		total += binomial_probability(proability_of_success, y, x_sucesses)

	total *= w

	print('yes') if total > 1 else print('no')