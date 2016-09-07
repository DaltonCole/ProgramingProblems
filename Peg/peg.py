import sys

line = input()
line += input()
line += input()
line += input()
line += input()
line += input()
line += input() + ' '

board = []

for i in range(7):
	board.append([])
	for j in range(7):
		board[i].append([])
		board[i][j] = line[(i*7) + j]

for i in range(7):
	for j in range(7):
		sys.stdout.write(board[i][j])
	sys.stdout.write("\n")