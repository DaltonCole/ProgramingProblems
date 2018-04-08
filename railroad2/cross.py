x, y = input().split()

x = int(x)
y = int(y)

total = x * 4
total += (y * 3)

if total % 2 == 0:
	print("possible")
else:
	print("impossible")