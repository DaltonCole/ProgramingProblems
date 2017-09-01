import math

while True:
	user_input = input()

	D, V = user_input.split(' ')
	D = int(D)
	V = int(V)

	if D == 0 and V == 0:
		break

	radius = D / 2
	radus_squared = radius ** 2

	# pi * r^2
	total_wood = math.pi * radus_squared * D

	middle_volume = total_wood - V

	d = math.sqrt((2 * middle_volume) - (D**2))

	print(d)



'''
middle_volume = 2*(((D + d) / 2) * (D/2 - d/2)) + (d * d)

(D/2 + d/2) * (D/2 - d/2) 
'''