from math import pi

while True:
	D, V = input().split()
	D = int(D)
	V = int(V)

	if D == 0 and V == 0:
		break

	print(pow(D*D*D - 6*V/pi, 1.0/3))