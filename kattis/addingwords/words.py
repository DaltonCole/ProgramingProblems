from sys import stdin

def equation(s):
	s = s[1:]
	new_s = ''
	for i in s:
		new_s += i
		new_s += ' '
	return new_s

var = {}
reverse_var = {}

for line in stdin:
	line = line.split()
	command = line[0]

	if command == "def":
		if line[1] in var:
			del reverse_var[var[line[1]]]
		

		var[line[1]] = int(line[-1])
		reverse_var[int(line[-1])] = line[1]

	elif command == "clear":
		var = {}
		reverse_var = {}

	elif command == "calc":
		formula = line[2:]
		eq = equation(line)
		try:
			total = int(var[line[1]])
			for num in range(0, len(formula), 2):
				if formula[num] == "=":
					break
				elif formula[num] == "-":
					total -= var[formula[num+1]]
				elif formula[num] == "+":
					total += var[formula[num+1]]
			print(str(eq) + reverse_var[total])
		except:
			print(str(eq) + "unknown")