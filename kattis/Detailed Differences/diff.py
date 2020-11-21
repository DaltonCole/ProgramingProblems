# For printing without newline
import sys

# Number of test cases
cases = int(input())

# For each test case
for i in range(cases):
	# Get strings a and b
	a = input()
	b = input()

	# Print a and b
	print(a)
	print(b)

	# Go through string to see differences
	for j in range(len(a)):
		# If same, print "."
		if a[j] == b[j]:
			sys.stdout.write('.')
		# Else, print "*"
		else:
			sys.stdout.write('*')
			
	sys.stdout.write("\n\n")