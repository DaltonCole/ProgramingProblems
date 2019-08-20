def calc(b, p):
	return (60 / p) * b

cases = int(input())

for i in range(cases):
	b, p = input().split()
	b = float(b)
	p = float(p)

	print("%.5f %.5f %.5f" % (calc(b - 1, p), calc(b, p), calc(b + 1, p)))