
bases = []

num = 0
while len(bases) != 50005:
	s = "{0:b}".format(num)

	if str(s) == str(s)[::-1]:
		bases.append(num)

	num += 1


with open("dict.py", 'w') as f:
	for i in range(len(bases)):
		f.write("d[{}] = {}\n".format(i, bases[i]))