letter_to_num = {}
letter_to_num[' '] = 0
letter_to_num['a'] = 1
letter_to_num['b'] = 2
letter_to_num['c'] = 3
letter_to_num['d'] = 4
letter_to_num['e'] = 5
letter_to_num['f'] = 6
letter_to_num['g'] = 7
letter_to_num['h'] = 8
letter_to_num['i'] = 9
letter_to_num['j'] = 10
letter_to_num['k'] = 11
letter_to_num['l'] = 12
letter_to_num['m'] = 13
letter_to_num['n'] = 14
letter_to_num['o'] = 15
letter_to_num['p'] = 16
letter_to_num['q'] = 17
letter_to_num['r'] = 18
letter_to_num['s'] = 19
letter_to_num['t'] = 20
letter_to_num['u'] = 21
letter_to_num['v'] = 22
letter_to_num['w'] = 23
letter_to_num['x'] = 24
letter_to_num['y'] = 25
letter_to_num['z'] = 26

num_to_letters = {}
num_to_letters[0] = ' '
num_to_letters[1] = 'a'
num_to_letters[2] = 'b'
num_to_letters[3] = 'c'
num_to_letters[4] = 'd'
num_to_letters[5] = 'e'
num_to_letters[6] = 'f'
num_to_letters[7] = 'g'
num_to_letters[8] = 'h'
num_to_letters[9] = 'i'
num_to_letters[10] = 'j'
num_to_letters[11] = 'k'
num_to_letters[12] = 'l'
num_to_letters[13] = 'm'
num_to_letters[14] = 'n'
num_to_letters[15] = 'o'
num_to_letters[16] = 'p'
num_to_letters[17] = 'q'
num_to_letters[18] = 'r'
num_to_letters[19] = 's'
num_to_letters[20] = 't'
num_to_letters[21] = 'u'
num_to_letters[22] = 'v'
num_to_letters[23] = 'w'
num_to_letters[24] = 'x'
num_to_letters[25] = 'y'
num_to_letters[26] = 'z'

cases = int(input())

for i in range(cases):
	line = input()
	method = line[0]

	line = line[2:]

	if method == 'e':
		previous = 0
		for char in line:
			current = (letter_to_num[char] + previous) % 27
			previous = current
			letter = num_to_letters[current]
			print(letter, end='')
		print()
	else:
		previous = 0
		for char in line:
			current = (27 + letter_to_num[char] - previous) % 27
			previous = letter_to_num[char]
			letter = num_to_letters[current]
			print(letter, end='')
		print()