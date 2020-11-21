cases = int(input())

for i in range(cases):
	num = input()

	start = num[:3]
	print(start, end='')
	print(start[::-1])

	mirrored = start + start[::-1]

	m_diff = abs(int(mirrored) - int(start))

	