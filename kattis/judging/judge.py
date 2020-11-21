cases = int(input())

words = {}

for i in range(cases):
	w = input()

	if w not in words:
		words[w] = [0,0]

	words[w][0] += 1

for i in range(cases):
	w = input()

	if w not in words:
		words[w] = [0,0]

	words[w][1] += 1

total = 0
for key, value in words.items():
	total += min(value[0], value[1])

print(total)