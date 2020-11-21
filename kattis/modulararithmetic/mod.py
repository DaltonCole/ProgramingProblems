while True:
	n, t = input().split()
	n = int(n)
	t = int(t)

	if n == 0 and t == 0:
		break

	for i in range(t):
		line = input()
		print("{}".format(int(eval(line)) % n))
