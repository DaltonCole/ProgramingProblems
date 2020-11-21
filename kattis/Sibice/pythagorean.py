import math

user_input = input()

total_matches, width, height = user_input.split(' ')

total_matches = int(total_matches)
width = int(width)
height = int(height)

max_size_match = math.sqrt((width ** 2) + (height ** 2))

for i in range(total_matches):
	match = int(input())
	if match <= max_size_match:
		print("DA")
	else:
		print("NE")
