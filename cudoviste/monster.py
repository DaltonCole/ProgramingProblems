rows, columns = input().split()

rows = int(rows)
columns = int(columns)

grid = []
for r in range(rows):
	line = input()
	grid.append([])
	for char in line:
		grid[-1].append(char)

zero_cars = 0
one_car = 0
two_cars = 0
three_cars = 0
four_cars = 0

for i in range(rows - 1):
	for j in range(columns - 1):
		cars = 0
		if grid[i][j] == '#':
			continue
		if grid[i+1][j] == '#':
			continue
		if grid[i][j+1] == '#':
			continue
		if grid[i+1][j+1] == '#':
			continue

		if grid[i][j] == 'X':
			cars += 1
		if grid[i+1][j] == 'X':
			cars += 1
		if grid[i][j+1] == 'X':
			cars += 1
		if grid[i+1][j+1] == 'X':
			cars += 1

		if cars == 0:
			zero_cars += 1
		elif cars == 1:
			one_car += 1
		elif cars == 2:
			two_cars += 1
		elif cars == 3:
			three_cars += 1
		elif cars == 4:
			four_cars += 1

print(zero_cars)
print(one_car)
print(two_cars)
print(three_cars)
print(four_cars)
