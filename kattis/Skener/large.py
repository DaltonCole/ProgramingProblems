r, c, zr, zc = input().split(" ")

r, c, zr, zc = int(r), int(c), int(zr), int(zc)

line = ""
output = ""

for i in range(r):
	line = input()

	for j in line:
		for k in range(zc):
			output += j

	for j in range(zr):
		print(output)

	output = ""