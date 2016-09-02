from datetime import datetime as DT

testCases = input()
testCases = int(testCases)

for i in range(testCases):
	line = input()
	c, x, y, z = line.split(' ')
	c = int(c)
	x = int(x)
	y = int(y)
	z = int(z)


	numbers = [x, y, z]
	numbers.sort()
	
	c = c % 4
	c = 100 * c + 2000

	numbers = []
	try:
		numbers.append(DT(c+x, y, z))
	except:
		pass
	try:
		numbers.append(DT(c+x, z, y))
	except:
		pass
	try:
		numbers.append(DT(c+y, x, z))
	except:
		pass
	try:
		numbers.append(DT(c+y, z, x))
	except:
		pass
	try:
		numbers.append(DT(c+z, x, y))
	except:
		pass
	try:
		numbers.append(DT(c+z, y, x))
	except:
		pass

	if len(numbers) != 0:
		date = min(numbers)
		print("%02d %02d %02d" % (date.year - c, date.month, date.day))
	else:
		print("-1")

