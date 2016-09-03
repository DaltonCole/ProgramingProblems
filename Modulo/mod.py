numbers = []

for i in range(10):
	n = int(input()) % 42

	if n not in numbers:
		numbers.append(n)

print(len(numbers))