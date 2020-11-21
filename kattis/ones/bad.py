import sys

with open('cheat.txt', 'w') as f:
	for line in range(1, 100001):
		number = int(line)

		if number % 100 == 0:
			print(number)

		if number % 2 == 0 or number % 5 == 0:
			continue

		ones = '1'
		answer = 0

		while True:
			if int(ones) % number == 0:
				answer = len(ones)
				break
			else:
				ones += '1'
		f.write('cheating_dict[' + str(number) + '] = ' + str(answer) + '\n')
