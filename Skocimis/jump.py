a, b, c = input().split()

a = int(a)
b = int(b)
c = int(c)

d = 0

if b - a > c - b:
	d = b - a
else:
	d = c - b

d -= 1

print(d)