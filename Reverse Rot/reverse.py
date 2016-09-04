def increment(letter):
	if letter == 'A':
		return 'B'
	elif letter == 'B':
		return 'C'
	elif letter == 'C':
		return 'D'
	elif letter == 'D':
		return 'E'
	elif letter == 'E':
		return 'F'
	elif letter == 'F':
		return 'G'
	elif letter == 'G':
		return 'H'
	elif letter == 'H':
		return 'I'
	elif letter == 'I':
		return 'J'
	elif letter == 'J':
		return 'K'
	elif letter == 'K':
		return 'L'
	elif letter == 'L':
		return 'M'
	elif letter == 'M':
		return 'N'
	elif letter == 'N':
		return 'O'
	elif letter == 'O':
		return 'P'
	elif letter == 'P':
		return 'Q'
	elif letter == 'Q':
		return 'R'
	elif letter == 'R':
		return 'S'
	elif letter == 'S':
		return 'T'
	elif letter == 'T':
		return 'U'
	elif letter == 'U':
		return 'V'
	elif letter == 'V':
		return 'W'
	elif letter == 'W':
		return 'X'
	elif letter == 'X':
		return 'Y'
	elif letter == 'Y':
		return 'Z'
	elif letter == 'Z':
		return '_'
	elif letter == '_':
		return '.'
	elif letter == '.':
		return 'A'
	
while True:
	add = input()

	if add == '0':
		break

	add, word = add.split(' ')
	add = int(add)
	word = list(word)

	for i in range(len(word)):
		for j in range(add):
			word[i] = increment(word[i])

	word = "".join(word)
	word = word[::-1]

	print(word)