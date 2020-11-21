from sys import stdin

for line in stdin:
	numbers = list(map(int, line.split()))

	for i in range(len(numbers)):
		total = 0
		for j in range(len(numbers)):
			if i != j:
				total += numbers[j]
		if total == numbers[i]:
			print(total)
			break