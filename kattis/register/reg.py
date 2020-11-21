max_values = [2, 3, 5, 7, 11, 13, 17, 19]

additive_values = [0]

'''
0 + 1 = 1

1 + 1 + 2 = 4

5 + 5 + 5 + 5 + 4 = 24

= (sum() * (n-1)) + (n-1)
'''

for val in max_values:
	tmp = (sum(additive_values) * (val - 1)) + (val - 1)
	additive_values.append(tmp)

additive_values = additive_values[1:]

for index, max_v in enumerate(max_values):
	additive_values[index] /= (max_v - 1)


values = list(map(int, input().split()))

total = 0

index = -1
for val, max_v in zip(values, max_values):
	index += 1

	overflow = max_v - val - 1

	total += (int(additive_values[index]) * overflow)

print(total)