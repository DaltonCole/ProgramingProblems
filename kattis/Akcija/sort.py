cases = int(input())

books = []

for i in range(cases):
	books.append(int(input()))

books.sort(reverse=True)

cost = 0

for i in range(len(books)):
	if i % 3 == 2:
		continue
	else:
		cost += books[i]

print(cost)