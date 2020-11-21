x1, y1 = input().split()
x1 = int(x1)
y1 = int(y1)

x2, y2 = input().split()
x2 = int(x2)
y2 = int(y2)

x3, y3 = input().split()
x3 = int(x3)
y3 = int(y3)

# Find diagnals
distance1 = pow(x1 - x2, 2) + pow(y1 - y2, 2)
distance2 = pow(x2 - x3, 2) + pow(y2 - y3, 2)
distance3 = pow(x3 - x1, 2) + pow(y3 - y1, 2)

if distance1 == max(distance1, distance2, distance3):
	pass
elif distance2 == max(distance1, distance2, distance3):
	x1, x2, x3 = x2, x3, x1
	y1, y2, y3 = y2, y3, y1
elif distance3 == max(distance1, distance2, distance3):
	x1, x2, x3 = x1, x3, x2
	y1, y2, y3 = y1, y3, y2

x4 = int((x1 + x2 + y1 - y2) / 2)
y4 = int((x2 - x1 + y1 + y2) / 2)

if x4 == x3 and y4 == y3:
	x4 = int((x1 + x2 + y2 - y1) / 2)
	y4 = int((x1 - x2 + y1 + y2) / 2)
	print(str(x4) + " " + str(y4))
else:
	print(str(x4) + " " + str(y4))

