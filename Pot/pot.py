# Number of inputs
numbers = int(input())

# Sum
total = 0

# For each input
for i in range(numbers):
	# Read in the input
	number = input()

	# Seperate the base from the exponent
	base = int(number[0:-1])
	exponent = int(number[-1])

	# Add to the total
	total += pow(base, exponent)

# Print total
print(total)