_ = input()

rooms = input().split()

students = []

for i in range(len(rooms)):
	students.append((int(rooms[i]), i + 1))


students.sort(reverse=True)

total = 0

for i in range(1, len(students)):
	total += students[i][0]

if total < students[0][0]:
	print("impossible")
else:
	for s, i in students:
		print(str(i), end=' ')
	print()

