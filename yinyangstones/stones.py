stones = input()

black = 0
white = 0

for char in stones:
	if char == 'W':
		white += 1
	else:
		black += 1

if white == black:
	print(1)
else:
	print(0)