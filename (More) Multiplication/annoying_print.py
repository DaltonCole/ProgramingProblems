def print_grid(grid):
	for i in grid:
		for j in i:
			print(j, end='')
		print()

while True:
	first_number, second_number = input().split()

	if first_number == "0":
		quit()

	grid = []

	for i in range((len(second_number) + 2) * 3 + len(second_number) - 1):
		grid.append([' '] * ((len(first_number) + 2) * 3 + len(first_number) - 1))

	# Get sides
	for row in grid:
		row[0] = "|"
		row[-1] = "|"

	# Get corners
	grid[0][0] = "+"
	grid[0][-1] = "+"
	grid[-1][0] = "+"
	grid[-1][-1] = "+"

	# Top, bottom row
	for i in range(1, len(grid[0]) - 1):
		grid[0][i] = '-'
		grid[-1][i] = '-'

	# Add rows of +---+---+ and top number
	start = 2
	top_number = 0
	still_top = True
	for i in range(start, len(grid), 4):
		for place in range(2, len(grid[i]) - 2, 4):
			if place + 3 != len(grid[i]):
				grid[i][place] = "+"
				grid[i][place + 1] = "-"
				grid[i][place + 2] = "-"
				grid[i][place + 3] = "-"
				# Add top number
				if still_top:
					grid[i-1][place + 2] = first_number[top_number]
					top_number += 1
					if top_number == len(first_number):
						still_top = False
			else:
				grid[i][place] = "+"


	# Add columns of |
	start = 3
	for i in range(start, len(grid) - start):
		for place in range(2, len(grid[i]) - 2, 4):
			if grid[i][place] != '+':
				grid[i][place] = '|'

	# Add diagnals inside boxes
	start = 3
	col_start = 3
	count = 0
	increment_count = False
	for i in range(start, len(grid) - start):
		for place in range(col_start, len(grid[i]) - col_start):
			if count == 0 and grid[i][place + 1] == '|':
				grid[i][place] = '/'
				increment_count = True
			if count == 1 and grid[i][place + 2] == '|':
				grid[i][place] = '/'
				increment_count = True
			if count == 2 and grid[i][place + 3] == '|' and place != len(grid[0]) - 4:
				grid[i][place] = '/'
				increment_count = True
		if increment_count:
			count = (count + 1) % 3
		increment_count = False

	# Add side numbers
	start = 4
	side_number = 0
	for i in range(start, len(grid), 4):
		grid[i][len(grid[0]) - 2] = second_number[side_number]
		side_number += 1
		if side_number == len(second_number):
			break

	# Fill top left of boxes
	start = 3
	col_start = 3
	top_number = 0
	side_number = 0
	for i in range(start, len(grid) - 5, 4):
		for place in range(col_start, len(grid[0]) - 5, 4):
			temp = int(first_number[top_number]) * int(second_number[side_number])
			temp = str(temp).zfill(2)
			grid[i][place] = temp[0]
			grid[i + 2][place + 2] = temp[1]

			top_number += 1
		top_number = 0
		side_number += 1

	# column number
	over_flow = 0
	for i in range(len(grid[0]) - 6, 2, -4):
		current_row = len(grid) - 2
		current_col = i
		count = 0
		break_now = False
		while not break_now: 
			current_col += 2
			current_row -= 2
			if grid[current_row - 1][current_col] == ' ' and grid[current_row][current_col + 2] == ' ':
				break_now = True
			if grid[current_row - 2][current_col] == ' ' and grid[current_row][current_col + 1] == ' ':
				break_now = True
			count += int(grid[current_row][current_col])
			if break_now:
				break

		count = str(count + int(over_flow)).zfill(2)
		grid[len(grid) - 2][i] = str(int(count[1]))
		grid[len(grid) - 2][i-2] = '/'
		over_flow = count[0]

	# Row number
	start = 4
	end = 4
	for i in range(len(grid) - end, start, -4):
		current_row = i
		current_col = 1
		count = 0
		break_now = False
		while not break_now:
			current_col += 2
			current_row -= 2
			if grid[current_row - 1][current_col] == ' ' and grid[current_row][current_col + 2] == ' ':
				break_now = True
			if grid[current_row - 2][current_col] == ' ' and grid[current_row][current_col + 1] == ' ':
				break_now = True
			count += int(grid[current_row][current_col])
			if break_now:
				break
		count = str(count + int(over_flow)).zfill(2)
		if count[1] != '0':
			grid[i][1] = str(int(count[1]))
			grid[i + 2][1] = '/'
		else:
			grid[i + 2][1] = ' '
		over_flow = count[0]

	print_grid(grid)