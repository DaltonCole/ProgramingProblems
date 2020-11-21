def hash_func(s):
	x = 0
	for c in s:
		x ^= ord(c)
	return x

def find_u_c(hash_dict):
	unique = 0
	collisions = 0
	for _, value in hash_dict.items():
		unique += len(set(value))
		for i in range(len(value)):
			for j in range(i+1, len(value)):
				if value[i] != value[j]:
					collisions += 1
	return unique, collisions

while True:
	cases = int(input())

	if cases == 0:
		break

	hash_dict = {}

	# Read in input
	for i in range(cases):
		s = input()
		# Find hash
		h = hash_func(s)
		# Add hash to list
		if h not in hash_dict:
			hash_dict[h] = []
		hash_dict[h].append(s)

	unique, collisions = find_u_c(hash_dict)

	print("{} {}".format(unique, collisions))