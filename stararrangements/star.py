from math import ceil

stars = int(input())

rows = []
differences = [-1, 0]

for i in range(2, int(ceil(stars / 2)) + 1):
	if stars % (i + i - 1) == 0 or (stars % (i + i - 1)) == i:
		rows.append((i, i - 1))
	if stars % i == 0:
		rows.append((i, i))


print("{}:".format(stars))

for i, j in rows:
	print("{},{}".format(i, j))
