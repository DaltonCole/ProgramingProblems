cases = int(input())

names = {}
for i in range(cases):
	direction, name = input().split()

	print(name, end=' ')

	try:
		if direction == "entry":
			names[name] += 1
		else: #direction == "exit":
			names[name] -= 1
	except:
		if direction == "entry":
			names[name] = 1
		else: #direction == "exit":
			names[name] = -1

	if names[name] == 1:
		print("entered")
	elif names[name] == 0:
		print("exited")
	elif names[name] > 1:
		print("entered (ANOMALY)")
		names[name] = 1
	else:
		print("exited (ANOMALY)")
		names[name] = 0