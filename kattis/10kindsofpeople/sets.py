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
							s.add((i, j))
							break
			# One
			elif line[i][j] == '1':
				# If one above is also 1
				if line[i-1][j] == '1':
					for enum, s in enumerate(ones):
						if (i-1, j) in s:
							in_a_set = True
							set_number = enum
							s.add((i, j))
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
								zeros[enum] = zeros[enum].union(zeros[set_number])
								del zeros[set_number]
							else:
								in_a_set = True
								s.add((i, j))
							break
			# One
			elif line[i][j] == '1':
				# If one to the left is also 1
				if line[i][j-1] == '1':
					for enum, s in enumerate(ones):
						if (i, j-1) in s:
							if in_a_set == True and enum != set_number:
								ones[enum] = ones[enum].union(ones[set_number])
								del ones[set_number]
							else:
								in_a_set = True
								s.add((i, j))
							break

		if not in_a_set:
			if line[i][j] == '0':
				zeros.append(set())
				zeros[-1].add((i, j))
			elif line[i][j] == '1':
				ones.append(set())
				ones[-1].add((i, j))

"""
print("Zeros:")
for i in zeros:
	print(i)
print()
print("Ones:")
for j in ones:
	print(j)
print()
"""

zero_dict = {}
ones_dict = {}

for i, s in enumerate(zeros):
	for j in s:
		zero_dict[j] = i

for i, s in enumerate(ones):
	for j in s:
		ones_dict[j] = i

cases = int(input())

for i in range(cases):
	x1, y1, x2, y2 = input().split()
	x1 = int(x1) - 1
	x2 = int(x2) - 1
	y1 = int(y1) - 1
	y2 = int(y2) - 1

	b = False
	d = False

	if (x1, y1) in zero_dict and (x2, y2) in zero_dict:
		if zero_dict[(x1, y1)] == zero_dict[(x2, y2)]:
			b = True
	
	if (x1, y1) in ones_dict and (x2, y2) in ones_dict:
		if ones_dict[(x1, y1)] == ones_dict[(x2, y2)]:
			d = True

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