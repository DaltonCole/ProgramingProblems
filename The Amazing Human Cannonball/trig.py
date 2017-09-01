import math

test_cases = int(input())

gravity = 9.81

for i in range(test_cases):
	user_input = input()

	v0, angle, x1, h1, h2 = user_input.split(' ')
	v0 = float(v0)
	angle = float(angle)
	x1 = float(x1)
	h1 = float(h1)
	h2 = float(h2)

	radians = math.radians(angle)

	t = x1 / (v0 * math.cos( radians ))

	y = (v0 * t * math.sin(radians)) - (.5 * gravity * t * t)

	if y < (h2 - 1) and y > (h1 + 1):
		print("Safe")
	else:
		print("Not Safe")