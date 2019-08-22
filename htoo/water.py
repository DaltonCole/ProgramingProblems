mol, multiplyer = list(input().split())
multiplyer = int(multiplyer)

atom_count = {}

parse_str = []
parse_num = []

number = '-'
for c in mol:
	# If no number
	if not c.isnumeric():
		parse_str.append(c)

		if number == '-':
			pass
		elif number != '':
			parse_num.append(int(number))
		else:
			parse_num.append(1)

		number = ''
	else:
		if number == '-':
			number = ''
		number += c

if number != '':
	parse_num.append(number)

if len(parse_str) != len(parse_num):
	parse_num.append(1)

for s, n in zip(parse_str, parse_num):
	if s not in atom_count:
		atom_count[s] = 0
	atom_count[s] += (int(n) * int(multiplyer))

####

mol = input()

want_count = {}

parse_str = []
parse_num = []

number = '-'
for c in mol:
	# If no number
	if not c.isnumeric():
		parse_str.append(c)

		if number == '-':
			pass
		elif number != '':
			parse_num.append(int(number))
		else:
			parse_num.append(1)

		number = ''
	else:
		if number == '-':
			number = ''
		number += c

if number != '':
	parse_num.append(number)

if len(parse_str) != len(parse_num):
	parse_num.append(1)

for s, n in zip(parse_str, parse_num):
	if s not in want_count:
		want_count[s] = 0
	want_count[s] += int(n)


intersection = {}

for key, value in want_count.items():
	if key not in atom_count:
		print(0)
		quit()

	intersection[key] = atom_count[key] // value

values = intersection.values()

print(min(values))