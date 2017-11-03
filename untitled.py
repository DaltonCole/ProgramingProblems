rotation = {}
rotation['-'] = '|'
rotation['|'] = '-'
rotation['+'] = '+'
rotation[' '] = ' '


first = True

while True:
	lines = int(input())
	if lines == 0:
		break
	else:
		if first == False:
			print()
		else:
			first = False
	art = []
	for i in range(lines):
		art.append(list(input()))

	# Find max line length
	max_length = 0
	for i in art:
		max_length = max(max_length, len(i))

	# Fill with zero's if not same length of longest
	for i in art:
		while len(i) != max_length:
			i.append(' ')

	rotated_art = []


	# Rotate
	for i in range(max_length):
		rotated_art.append([])
		for j in range(len(art)-1, -1, -1):
			rotated_art[i].append(rotation[art[j][i]])

	# Remove ending spaces
	for i in rotated_art:
		while True:
			if i[-1] == ' ':
				del i[-1]
			else:
				break

	# Print
	for i in rotated_art:
		for j in i:
			print(j, end='')
		print()
