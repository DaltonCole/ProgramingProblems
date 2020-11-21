ases = int(input())

total = 0

for i in range(cases):
	move = input()

	if 'CD' not in move:
		total += 1

print(total)