
correction = {}
correction['B'] = '8'
correction['G'] = 'C'
correction['I'] = '1'
correction['O'] = '0'
correction['Q'] = '0'
correction['S'] = '5'
correction['U'] = 'V'
correction['Y'] = 'V'
correction['Z'] = '2'

base_27 = "0123456789ACDEFHJKLMNPRTVWX"

char_to_num = {}

count = 0
for i in base_27:
	char_to_num[i] = count
	count += 1

cases = int(input())

for i in range(cases):
	number, fbi = input().split()

	new_fbi = ''

	for char in fbi:
		if char not in correction:
			new_fbi += char
		else:
			new_fbi += correction[char]

	total = (2*char_to_num[new_fbi[0]])
	total += (4*char_to_num[new_fbi[1]])
	total += (5*char_to_num[new_fbi[2]])
	total += (7*char_to_num[new_fbi[3]])
	total += (8*char_to_num[new_fbi[4]])
	total += (10*char_to_num[new_fbi[5]])
	total += (11*char_to_num[new_fbi[6]])
	total += (13*char_to_num[new_fbi[7]])

	new_num = total % 27

	if new_num != char_to_num[new_fbi[8]]:
		print(str(i + 1) + " Invalid")
	else:
		total = 0
		count = 0
		for num in range(7, -1, -1):
			total += char_to_num[new_fbi[num]] * pow(27, count)
			count += 1
		print(str(i + 1), end=' ')
		print(total)