import math

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
	middle_x = (coordinate_pairs[0][0] + coordinate_pairs[1][0] + coordinate_pairs[2][0]) / 3
	middle_y = (coordinate_pairs[0][1] + coordinate_pairs[1][1] + coordinate_pairs[2][1]) / 3

	dx1 = coordinate_pairs[1][0] - coordinate_pairs[0][0]
	dy1 = coordinate_pairs[1][1] - coordinate_pairs[0][1]

	dx2 = coordinate_pairs[1][0] - coordinate_pairs[2][0]
	dy2 = coordinate_pairs[1][1] - coordinate_pairs[2][1]

	cross = (dx1 * dx2) + (dy1 * dy2)
	cross /= (pow(pow(dx1, 2) + pow(dy1, 2), .5) * pow(pow(dx2, 2) + pow(dy2, 2), .5))

	cross = math.acos(cross)
	print(cross)

	if cross < 0:
		print("CW ", end='')
	else:
		print("CCW ", end='')
	####################

	print(abs(area))
