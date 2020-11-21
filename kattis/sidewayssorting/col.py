while True:
	row, col = input().split()

	row = int(row)
	col = int(col)

	if row == 0 and col == 0:
		break

	words = []

	for i in range(row):
		temp = input()
		words.append(temp)

	sideways = []

	for i in range(col):
		new_word = ''
		for j in range(row):
			new_word += words[j][i]
		sideways.append(new_word)

	sideways = sorted(sideways, key=str.lower)

	for i in range(row):
		for j in range(col):
			print(sideways[j][i], end='')
		print()
	print()
	print()