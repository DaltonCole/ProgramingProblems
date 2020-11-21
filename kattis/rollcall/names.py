from sys import stdin
from collections import OrderedDict

roll = {}
last_names = set()

for line in stdin:
	first, last = line.split()

	last_names.add(last)

	if last not in roll:
		roll[last] = [first]
	else:
		roll[last].append(first)

last_names = list(last_names)
last_names.sort()

for name in last_names:
	if len(roll[name]) != 1:
		test = roll[name]
		test.sort()
		for i in test:
			print(i)
	else:
		for i in roll[name]:
			print(str(i) + " " + str(name))
