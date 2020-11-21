from math import sqrt

cases = int(input())

for i in range(cases):
	book_x, book_y = input().split()
	book_x = float(book_x)
	book_y = float(book_y)

	books = int(input())

	light = False

	for j in range(books):
		x, y = input().split()
		x = float(x)
		y = float(y)

		if sqrt(pow((x - book_x), 2) + pow((y - book_y), 2)) <= 8:
			light = True
	if light:
		print("light a candle")
	else:
		print("curse the darkness")