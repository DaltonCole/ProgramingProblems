def print_numbers(num):
	for i in num:
		print(i, end=' ')
	print()

numbers = input().split()

numbers = list(map(int, numbers))

sorted_numbers = []
for i in numbers:
	sorted_numbers.append(i)
sorted_numbers.sort()

while numbers != sorted_numbers:
	for i in range(len(numbers) - 1):
		if numbers[i] > numbers[i + 1]:
			numbers[i], numbers[i+1] = numbers[i+1], numbers[i]
			print_numbers(numbers)
