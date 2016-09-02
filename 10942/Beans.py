def valid_date(year, month, day):
	thirty_one = [1, 3, 5, 7, 8, 10, 12]
	thirty = [4, 6, 9, 11]
	feburary = 2

	if month > 12 or day > 31:
		return False
	if month in thirty_one:
		if day > 31:
			return False
	elif month in thirty:
		if day > 30:
			return False
	# No Leap Year
	elif month == feburary:
		if year % 4 != 0 or (year % 100 == 0 and year % 400 != 0):
			if day > 28:
				return False
		else:
			if day > 29:
				return False
	return True


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
	c = 100 * c

	if(valid_date((c+x), y, z)):
		year = x
	if(valid_date((c+x), z, y)):
		if y > z:
			year = x
	if(valid_date((c+y), x, z)):
		if year > y:
			year = y
	if(valid_date((c+y), z, x)):
		if z > 
	if(valid_date((c+z), x, y)):

	if(valid_date((c+z), y, x)):

	if(valid_date((c+numbers[0]), numbers[1], numbers[2])):
		print("%02d %02d %02d" % (numbers[0], numbers[1], numbers[2]))
	else:
		print("-1")


def valid_date(year, month, day):
	thirty_one = [1, 3, 5, 7, 8, 10, 12]
	thirty = [4, 6, 9, 11]
	feburary = 2

	if month > 12 or day > 31:
		return false
	if month in thirty_one:
		if day > 31:
			return false
	elif month in thirty:
		if day > 30:
			return false
	# No Leap Year
	elif month == feburary:
		if year % 4 != 0 or (year % 100 == 0 and year % 400 != 0):
			if day > 28:
				return false
		else:
			if day > 29:
				return false
	return true