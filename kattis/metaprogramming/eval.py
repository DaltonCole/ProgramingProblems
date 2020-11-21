rom sys import stdin

d = {}

for line in stdin:
	line = line.split()

	if line[0] == "define":
		d[line[2]] = int(line[1])
	else:
		try:
			if line[2] == ">":
				if d[line[1]] > d[line[3]]:
					print("true")
				else:
					print("false")
			elif line[2] == "<":
				if d[line[1]] < d[line[3]]:
					print("true")
				else:
					print("false")
			elif line[2] == "=":
				if d[line[1]] == d[line[3]]:
					print("true")
				else:
					print("false")
		except:
			print("undefined")