p, n, u = input().split()
p = int(p)
n = int(n)
u = int(u)

total = (3 * p) + (2 * n) + u

if total < 2:
	print("Water")
else:
	if total >= 8:
		print("Cinnamon Roll", end='')
	elif total >= 5:
		print("Danish", end='')
	else:
		print("Donut", end='')

	print(" or ", end='')

	if total >= 6:
		print("Chocolate")
	elif total >= 3:
		print("Coffee")
	else:
		print("Water")

