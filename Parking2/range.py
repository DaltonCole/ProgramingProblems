test_cases = int(input())

for i in range(test_cases):
	spam = input()
	numbers = input().split(' ')

	numbers = list(map(int, numbers))

	numbers.sort()

	print(2 * (numbers[-1] - numbers[0]))