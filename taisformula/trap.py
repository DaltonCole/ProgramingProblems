total = 0

cases = int(input()) - 1
l_x, l_y = list(map(float, input().split()))

for _ in range(cases):
	x , y = list(map(float, input().split()))

	total += (((y + l_y) / 2) * (x - l_x) / 1000)
	l_x = x
	l_y = y


print('{0:.6f}'.format(total).rstrip('0').rstrip('.'))