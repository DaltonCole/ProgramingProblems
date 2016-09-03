# Don't care about how many elements, we have len in python
junk = input()

# Correct sequence
sequence = input()

# Everyone's sequence
Adrian = "ABC"
Bruno = "BABC"
Goran = "CCAABB"

# Set correctness to 0
a = 0
b = 0
c = 0

# Go down the string
for i in range(len(sequence)):
	if sequence[i] == Adrian[i % 3]:
		a += 1
	if sequence[i] == Bruno[i % 4]:
		b += 1
	if sequence[i] == Goran[i % 6]:
		c += 1

# Create a list to find max easily
l = [a,b,c]

# Print max number in l
print(max(l))

# Print names of people who have the max points
if(a == max(l)):
	print("Adrian")

if(b == max(l)):
	print("Bruno")

if(c == max(l)):
	print("Goran")