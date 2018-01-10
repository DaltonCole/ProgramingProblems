input()

line = input().split()

line = list(map(int, line))

s = 0
count = 0

for i in line:
	if i >= 0:
		s += i
		count += 1

print(s / count)