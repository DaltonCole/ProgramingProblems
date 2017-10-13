import sys

for line in sys.stdin:
	number = int(line)

	if number == 0:
		break

	# Convert number to binary
	binary = "{0:b}".format(number - 1)

	# Reverse number
	binary = binary[::-1]

	s = ''

	# For each bit
	for i in range(len(binary)):
		# If bit is set
		if binary[i] == '1':
			# Do 3 ** bit position
			s += ', ' + str(pow(3, i))

	s = '{' + s[1:] + ' }'

	print(s)