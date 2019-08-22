

for i in range(1, 11):
	count = 0

	for b in range(pow(2, i)):
		if '11' not in "{0:b}".format(b):
			count += 1

	print('{}:\t {}'.format(i, count))