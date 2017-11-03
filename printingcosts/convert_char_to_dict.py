d = {}

with open('characters', 'r') as f:
	for line in f:
		line = line.split()
		for i in range(0, len(line), 2):
			d[line[i]] = int(line[i+1])

with open('dict_char', 'w') as f:
	for key, value in d.items():
		f.write("toner['" + str(key) + "'] = " + str(value) + "\n")