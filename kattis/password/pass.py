cases = int(input())

all_nums = []

for i in range(cases):
	_, num = input().split()
	num = float(num)
	all_nums.append(num)

all_nums.sort(reverse=True)

guesses = 0

for i in range(cases):
	guesses += (i + 1) * all_nums[i]

print(guesses)