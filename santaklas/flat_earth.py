from math import cos, floor, pi

def santa(height, angle):
	if angle <= 180:
		print('safe')
		return

	if angle <= 270:
		angle = 270 - angle
	else:
		angle -= 270

	if angle == 0:
		print(height)
		return

	# Trig time!

	hypo = height / cos(pi * (angle / 180))
	hypo = floor(hypo)

	print(hypo)

'''
cos(angle in radians) = adj / hyp
hyp = adjacent / cos(angle in radians)
'''

h, a = list(map(int, input().split()))
santa(h, a)
