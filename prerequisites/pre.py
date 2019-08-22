while True:
	line = input()
	if line.startswith('0'):
		break

	k, m = list(map(int, line.split()))

	taken_cources = set(input().split())

	accepted = True

	for i in range(m):
		row = input().split()
		c = int(row[0])
		r = int(row[1])

		row = row[2:]

		count = 0
		for item in row:
			if item in taken_cources:
				count += 1

		if count < r:
			accepted = False

	if accepted:
		print('yes')
	else:
		print('no')