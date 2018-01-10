while True:
	num = int(input())
	if num == 0:
		break

	divisible = []
	less = []
	greater = []

	for i in range(num):
		line = input().split()
		if line[0] == "less":
			less.append(int(line[2]))
		elif line[0] == "greater":
			greater.append(int(line[2]))
		else:
			divisible.append(int(line[2]))

	if len(less) == 0:
		print("infinite")
		continue

	less.sort()
	greater.sort()

	largest = less[0]
	smallest = 0
	if len(greater) != 0:
		smallest = greater[-1]

	valid = [i for i in range(smallest + 1, largest)]

	for div in divisible:
		for i in range(len(valid)):
			if valid[i] % div != 0:
				valid[i] = -1

	good = []
	for i in valid:
		if i != -1:
			good.append(i)

	if len(good) == 0:
		print("none")
	else:
		for i in good:
			print(i, end=' ')
		print()