cases = int(input())

for i in range(cases):
	a, b = input().split()
	a = int(a)
	b = int(b)

	sum_all = int(b*(b+1) / 2)

	even = b * (b + 1)

	odd = b * b 

	print(a, end=' ')
	print(sum_all, end = ' ')
	
	print(odd,end=' ')
	print(even)