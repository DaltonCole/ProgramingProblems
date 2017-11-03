def find_islands(numbers):
	count = 1

	smallest = min(numbers)

	island = []

	for num in numbers:
		if num == smallest and island != []:
			count += find_islands(island)
			island = []
		elif num == smallest:
			pass
		else:
			island.append(num)
	if island != []:
		count += find_islands(island)

	return count


cases = int(input())

for i in range(cases):
	numbers = input().split()
	numbers = numbers[1:]

	numbers = list(map(int, numbers))

	print(i + 1, end=' ')
	print(find_islands(numbers) - 1)