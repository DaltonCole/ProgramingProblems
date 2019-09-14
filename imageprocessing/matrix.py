h1, w1, h2, w2 = list(map(int, input().split()))

matrix = []
kernal = []

for _ in range(h1):
	matrix.append(list(map(int, input().split())))

for _ in range(h2):
	kernal.append(list(map(int, input().split())))
	kernal[-1].reverse()

kernal.reverse()

result = []

# For each row in result
for a in range(h1 - h2 + 1):
	rrow = []
	# For each column in result
	for b in range(w1 - w2 + 1):
		value = 0
		for i in range(h2):
			for j in range(w2):
				value += matrix[a + i][b + j] * kernal[i][j]
		rrow.append(value)
	result.append(rrow)

for r in result:
	for c in r:
		print(c,end=' ')
	print()