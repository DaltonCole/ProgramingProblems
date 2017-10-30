cases = int(input())

for i in range(cases):
	_, num = input().split()

	print(i + 1, end=' ')
	try:
		print(int(num, 8), end = ' ')
	except:
		print(0, end = ' ')

	print(int(num), end=' ')

	print(int(num, 16))