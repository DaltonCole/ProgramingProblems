a = input()
b = input()

a = a[::-1]
b = b[::-1]

x = ''
y = ''

for i, j in zip(a, b):
	if i > j:
		x += i
	elif j > i:
		y += j
	else:
		x += i
		y += j

a = a[::-1]
b = b[::-1]
x = x[::-1]
y = y[::-1]

if len(a) > len(b):
	diff = len(a) - len(b)
	for i in range(diff):
		x = a[i] + x

if len(a) < len(b):
	diff = len(b) - len(a)
	for i in range(diff):
		y = b[i] + y

if x == '':
	print("YODA")
else:
	print(int(x))

if y == '':
	print("YODA")
else:
	print(int(y))