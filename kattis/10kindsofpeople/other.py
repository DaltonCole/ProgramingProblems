rows, columns = input().split()

rows = int(rows)
columns = int(columns)

zeros = []
ones = []
line = []

for i in range(rows):
	line.append(input())
	for j in range(columns):
		in_a_set = False
		set_number = 0
		# Check up
		if i != 0:
			# Zero
			if line[i][j] == '0':
				# If one above is also 0
				if line[i-1][j] == '0':
					for enum, s in enumerate(zeros):
						if (i-1, j) in s:
							in_a_set = True
							set_number = enum
							s.append((i, j))
							break
			# One
			elif line[i][j] == '1':
				# If one above is also 1
				if line[i-1][j] == '1':
					for enum, s in enumerate(ones):
						if (i-1, j) in s:
							in_a_set = True
							set_number = enum
							s.append((i, j))
							break
		# Check left
		if j != 0:
			# Zero
			if line[i][j] == '0':
				# If one to the left is also 0
				if line[i][j-1] == '0':
					for enum, s in enumerate(zeros):
						if (i, j-1) in s:
							if in_a_set == True and enum != set_number:
								zeros[enum] += zeros[set_number]
								del zeros[set_number]
							else:
								in_a_set = True
								s.append((i, j))
							break
			# One
			elif line[i][j] == '1':
				# If one to the left is also 1
				if line[i][j-1] == '1':
					for enum, s in enumerate(ones):
						if (i, j-1) in s:
							if in_a_set == True and enum != set_number:
								ones[enum] += ones[set_number]
								del ones[set_number]
							else:
								in_a_set = True
								s.append((i, j))
							break

		if not in_a_set:
			if line[i][j] == '0':
				zeros.append([(i, j)])
			elif line[i][j] == '1':
				ones.append([(i, j)])

for i in range(len(zeros)):
	zeros[i] = set(zeros[i])

for i in range(len(ones)):
	ones[i] = set(ones[i])

cases = int(input())

for i in range(cases):
	x1, y1, x2, y2 = input().split()
	x1 = int(x1) - 1
	x2 = int(x2) - 1
	y1 = int(y1) - 1
	y2 = int(y2) - 1

	b = False
	d = False

	for s in zeros:
		if (x1, y1) in s:
			if (x2, y2) in s:
				b = True
				break

	for s in ones:
		if (x1, y1) in s:
			if (x2, y2) in s:
				d = True
				break
	if b == False and d == False:
		print("neither")
	elif b == True:
		print("binary")
	elif d == True:
		print("decimal")