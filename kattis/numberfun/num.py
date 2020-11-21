def all(x, y, z):
	if x + y == z:
		return True
	if x * y == z:
		return True
	if x - y == z:
		return True
	if y - x == z:
		return True
	if x / y == z:
		return True
	if y / x == z:
		return True
	return False

cases = int(input())

for i in range(cases):
	a, b, c = input().split()
	a = int(a)
	b = int(b)
	c = int(c)

	if all(a, b, c):
		print("Possible")
	else:
		print("Impossible")
