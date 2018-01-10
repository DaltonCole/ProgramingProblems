cases = int(input())

pieces = []
max_n = 0

for i in range(cases):
	x, y = input().split()
	x = int(x)
	y = int(y)
	max_n = max(max_n, x, y)
	pieces.append((x, y))

if max_n != len(pieces) - 1:
	print("INCORRECT")
	quit()

for i in range(cases):
	for j in range(i + 1, cases):
		if pieces[i][0] == pieces[j][0]:
			print("INCORRECT")
			quit()
		if pieces[i][1] == pieces[j][1]:
			print("INCORRECT")
			quit()
		if abs(pieces[i][0] - pieces[j][0]) == abs(pieces[i][1] - pieces[j][1]):
			print("INCORRECT")
			quit()

print("CORRECT")

