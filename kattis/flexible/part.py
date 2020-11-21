width, splits = input().split()
width = int(width)
splits = int(splits)

partitions = []

if splits != 0:
	partitions = list(map(int, input().split()))

partitions = [0] + partitions + [width]

possibiles = set()

for i in partitions:
	for j in partitions:
		if i < j:
			possibiles.add(j - i)

possibiles = sorted(possibiles)

for i in possibiles:
	print("{} ".format(i), end='')
print()