import sys

letter_to_morse = {}
letter_to_morse['A'] = '.-'
letter_to_morse['B'] = '-...'
letter_to_morse['C'] = '-.-.'
letter_to_morse['D'] = '-..'
letter_to_morse['E'] = '.'
letter_to_morse['F'] = '..-.'
letter_to_morse['G'] = '--.'
letter_to_morse['H'] = '....'
letter_to_morse['I'] = '..'
letter_to_morse['J'] = '.---'
letter_to_morse['K'] = '-.-'
letter_to_morse['L'] = '.-..'
letter_to_morse['M'] = '--'
letter_to_morse['N'] = '-.'
letter_to_morse['O'] = '---'
letter_to_morse['P'] = '.--.'
letter_to_morse['Q'] = '--.-'
letter_to_morse['R'] = '.-.'
letter_to_morse['S'] = '...'
letter_to_morse['T'] = '-'
letter_to_morse['U'] = '..-'
letter_to_morse['V'] = '...-'
letter_to_morse['W'] = '.--'
letter_to_morse['X'] = '-..-'
letter_to_morse['Y'] = '-.--'
letter_to_morse['Z'] = '--..'
letter_to_morse['_'] = '..--'
letter_to_morse[','] = '.-.-'
letter_to_morse['.'] = '---.'
letter_to_morse['?'] = '----'

morse_to_letter = {}

for key, value in letter_to_morse.items():
	morse_to_letter[value] = key

 	 
for line in sys.stdin:
	line = line.split()[0]
	length = []
	morse = []
	for char in line:
		length.append(len(letter_to_morse[char]))
		for c in letter_to_morse[char]:
			morse.append(c)

	count = 0
	for i in reversed(length):
		new_word = ''
		for j in range(i):
			new_word += morse[count]
			count += 1
		print(morse_to_letter[new_word], end='')
	print()