numbers = input().split()
numbers = list(map(int, numbers))

order = input()

numbers.sort()

for i in range(3):
	if order[i] == 'A':
		print(numbers[0], end=' ')
	elif order[i] == 'B':
		print(numbers[1], end=' ')
	elif order[i] == 'C':
		print(numbers[2], end =' ')


print()