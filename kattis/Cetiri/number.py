a, b, c = input().split()
a = int(a)
b = int(b)
c = int(c)

t = [a,b,c]

t.sort()

if t[1] - t[0] == t[2] - t[1]:
	print(int((t[2] + t[1] - t[0])))
elif t[1] - t[0] == 2 * (t[2] - t[1]):
	print(int(t[0]) + t[2] - t[1])
else:
	print(int(t[1] + t[1] - t[0]))
