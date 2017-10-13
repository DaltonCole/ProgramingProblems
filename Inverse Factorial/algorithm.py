def primes(n):
	primfac = []
	d = 2
	while d*d <= n:
		while (n % d) == 0:
			primfac.append(d)  # supposing you want multiple factors repeated
			n //= d
		d += 1
	if n > 1:
	   primfac.append(n)
	return primfac

def dividable(original, number):
	p = primes(number)

	for i in p:
		if original % i != 0:
			return False

	return True

original = int(input())
big = "{0:b}".format(original)
reverse_big = big[::-1]

trailing_zeros = 0

for c in reverse_big:
	if c == '0':
		trailing_zeros += 1
	else:
		break

zero_int = "{0:b}".format(trailing_zeros)

count = 0

while(count != "{0:b}".format(trailing_zeros).count('1')):
	count += 1
	trailing_zeros += 1


if dividable(original, trailing_zeros + 1):
	print(trailing_zeros + 1)
else:
	print(trailing_zeros)

