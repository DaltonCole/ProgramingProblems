import itertools


number = input()

number = itertools.permutations(number)

for i in number:
	print(i)

print(number)

"""
number = list(number)

# Find smallest
next_index = -1
for i in range(len(number) - 1, 0, -1):
	if number[i-1] < number[i]:
		next_index = i
		break

if next_index == -1:
	print(0)
else:
	# Swap
	number[i-1], number[i] = number[i], number[i-1]

	# Order
	temp = number[i:]
	temp.sort()

	for i in range(next_index, len(number)):
		number[i] = temp[i - next_index]

	print(''.join(number))
"""