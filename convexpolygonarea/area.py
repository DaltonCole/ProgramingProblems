cases = int(input())

for i in range(cases):
	points = input().split()[1:]
	points = list(map(int, points))

	x = []
	y = []
	for j in range(0, len(points), 2):
		x.append(points[j])
		y.append(points[j+1])
	
	area = 0

	for z in range(len(x)):
		other = (z + 1) % len(x)
		area += (x[z] * y[other]) - (x[other] * y[z])

	area = abs(area / 2)

	if area.is_integer():
		area = int(area)

	print(area)