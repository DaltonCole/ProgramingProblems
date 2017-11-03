def order(first, second):
	if first[-4] < second[-4]:
		return -1
	elif second[-4] < first[-4]:
		return 1
	else:
		if first < second:
			return -1
		if second < first:
			return 1
	return 0

def change_str(s):
	new_s = s[-4:]

	if s[:2] == '12':
		new_s += '00'
		new_s += s[2:-4]
	elif s[2] == ':':
		new_s += 'z'
		new_s += s[:-4]
	else:
		new_s += s[:-4]


	return new_s

while True:
	cases = int(input())

	if cases == 0:
		break

	times = []
	for i in range(cases):
		times.append(input())

	times.sort(key=change_str)

	for i in times:
		print(i)
	print()