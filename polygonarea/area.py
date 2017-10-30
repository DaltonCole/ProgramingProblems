while True:
	points = int(input())
	if points == 0:
		break

	coordinate_pairs = []

	for i in range(points):
		x, y = input().split()
		coordinate_pairs.append((int(x), int(y)))

	area_sum = 0

	for i in range(points):
		area_sum += (coordinate_pairs[i][0] * coordinate_pairs[(i+1) % points][1]) - (coordinate_pairs[i][1] * coordinate_pairs[(i+1) % points][0])

	area = area_sum / 2

	### PROBLEM AREA ###
	dx1 = coordinate_pairs[1][0] - coordinate_pairs[0][0]
	dy1 = coordinate_pairs[1][1] - coordinate_pairs[0][1]

	dx2 = coordinate_pairs[2][0] - coordinate_pairs[1][0]
	dy2 = coordinate_pairs[2][1] - coordinate_pairs[1][1]

	cross = (dx1 * dy2) - (dy1 * dx2)

	if cross < 0:
		print("CW ", end='')
	else:
		print("CCW ", end='')
	####################

	print(abs(area))
