cases = int(input())

for i in range(cases):
	rows, columns = input().split()

	grid = []

	for r in range(int(rows)):
		grid.append(input())
		
	print("Test " + str(i + 1))
	for x in reversed(grid):
		print(x[::-1])

	