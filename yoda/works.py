a = list(input())
b = list(input())

if len(a) < len(b):
	while len(a) != len(b):
		a = ['0'] + a
else:
	while len(a) != len(b):
		b = ['0'] + b

for i in range(len(a)):
	if a[i] < b[i]:
		a[i] = -1
	elif a[i] > b[i]:
		b[i] = -1

a = [x for x in a if x != -1]
b = [x for x in b if x != -1]

a = ''.join(a)
b = ''.join(b)

if a != '':
	print(int(a))
else:
	print('yoda')

if b != '':
	print(int(b))
else:
	print('yoda')
