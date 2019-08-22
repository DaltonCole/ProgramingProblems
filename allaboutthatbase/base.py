base_str = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
			'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0']
base_num = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
			26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36]

cases = int(input())

for _ in range(cases):
	line = input()
	x, op, y, _, z = line.split()

	acceptable_bases = []

	for key, value in zip(base_str, base_num):
		if key == '1':
			# 1 case
			good_1 = True
			for w in x:
				if w != '1':
					good_1 = False
					break
			if good_1:
				for w in y:
					if w != '1':
						good_1 = False
						break
			if good_1:
				for w in z:
					if w != '1':
						good_1 = False
						break

			if good_1:
				if op == '+':
					if (len(x) + len(y)) == len(z):
						acceptable_bases.append(key)
				elif op == '-':
					if (len(x) - len(y)) == len(z):
						acceptable_bases.append(key)
				elif op == '*':
					if (len(x) * len(y)) == len(z):
						acceptable_bases.append(key)
				elif op == '/':
					if (len(x) / len(y)) == len(z):
						acceptable_bases.append(key)

			continue


		try:
			if op == '+':
				if (int(x, value) + int(y, value)) == int(z, value):
					acceptable_bases.append(key)
			elif op == '-':
				if (int(x, value) - int(y, value)) == int(z, value):
					acceptable_bases.append(key)
			elif op == '*':
				if (int(x, value) * int(y, value)) == int(z, value):
					acceptable_bases.append(key)
			elif op == '/':
				if (int(x, value) / int(y, value)) == int(z, value):
					acceptable_bases.append(key)
		except:
			pass

	acc_str = ''
	for i in acceptable_bases:
		acc_str += i

	if acc_str == '':
		print('invalid')
	else:
		print(acc_str)