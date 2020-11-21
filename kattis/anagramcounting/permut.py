from sys import stdin

fact = {}
t = 1
for i in range(1, 101):
	t *= i
	fact[i] = t


for line in stdin:
	line = str(line.split()[0])
	repeats = {}
	for char in line:
		if char not in repeats:
			repeats[char] = 1
		else:
			repeats[char] += 1

	total = fact[len(line)]

	for key, value in repeats.items():
		if value != 1:
			total = total // fact[value]

	print(int(total))