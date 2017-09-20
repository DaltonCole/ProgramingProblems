# For each test case
test_cases = int(input())

for i in range(test_cases):
	_ = input()

	# Get knots
	all_knots = input().split()

	# Get ints of blue knots
	blue_knots = [int(x[0:-1]) for x in all_knots if x[-1] == 'B']

	# Get ints of red knots
	red_knots = [int(x[0:-1]) for x in all_knots if x[-1] == 'R']

	# If only one color knot, zero length
	if len(blue_knots) == 0 or len(red_knots) == 0:
		print("Case #" + str(i + 1) +": 0")
		continue

	# Sort from longest to shortest
	blue_knots.sort(reverse=True)
	red_knots.sort(reverse=True)

	### Find length ###
	total_length = 0

	# Take a string off both lists
	while len(blue_knots) > 0 and len(red_knots) > 0:
		# Add it to total length
		total_length += blue_knots[0]
		total_length += red_knots[0]

		# Remove first string from both lists
		blue_knots.pop(0)
		red_knots.pop(0)
		# Knots take up 2 cm
		total_length -= 2

	print("Case #" + str(i + 1) +": " + str(total_length))