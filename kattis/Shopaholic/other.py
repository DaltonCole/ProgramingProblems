_ = input()

numbers = input().split()

numbers = list(map(int, numbers))

numbers.sort(reverse=True)

total = 0

for i in range(2, len(numbers), 3):
	total += numbers[i]

print(total)