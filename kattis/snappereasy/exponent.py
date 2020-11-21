cases = int(input())

for i in range(cases):
	n, k = input().split()
	n = int(n)
	k = int(k)

	exp = pow(2, n)

	modded = k % exp

	print("Case #" + str(i + 1) + ": ", end='')
	if modded == exp - 1:
		print("ON")
	else:
		print("OFF")m