knights = []

for i in range(5):
	line = input()
	for j in range(5):
		if line[j] == 'k':
			knights.append((i, j))

if len(knights) != 9:
	print("invalid")
	quit()

for i in range(9):
	for j in range(9):
		if i != j:
			if knights[i][0] - 1 == knights[j][0] and knights[i][1] - 2 == knights[j][1]:
				print("invalid")
				quit()
			elif knights[i][0] - 1 == knights[j][0] and knights[i][1] + 2 == knights[j][1]:
				print("invalid")
				quit()
			elif knights[i][0] + 1 == knights[j][0] and knights[i][1] - 2 == knights[j][1]:
				print("invalid")
				quit()
			elif knights[i][0] + 1 == knights[j][0] and knights[i][1] + 2 == knights[j][1]:
				print("invalid")
				quit()
			elif knights[i][0] - 2 == knights[j][0] and knights[i][1] - 1 == knights[j][1]:
				print("invalid")
				quit()
			elif knights[i][0] - 2 == knights[j][0] and knights[i][1] + 1 == knights[j][1]:
				print("invalid")
				quit()
			elif knights[i][0] + 2 == knights[j][0] and knights[i][1] - 1 == knights[j][1]:
				print("invalid")
				quit()
			elif knights[i][0] + 2 == knights[j][0] and knights[i][1] + 1 == knights[j][1]:
				print("invalid")
				quit()
print("valid")