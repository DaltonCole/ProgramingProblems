# Any Y only has one X
def injective(domain, co_domain, mappings):
	if len(set(mappings.values())) == len(mappings.values()):
		return True
	return False

# Every Y is used
def surjective(domain, co_domain, mappings):
	if len(set(mappings.values())) == len(set(co_domain)):
		return True
	return False


from sys import stdin

input_str = ''

for line in stdin:
	input_str += (line)

input_str = input_str.split('\n')

while len(input_str) > 3:
	try:
		domain = input_str[0]
		co_domain = input_str[1]
		mapping = int(input_str[2])

		mappings = []

		for i in range(mapping):
			mappings.append(input_str[3 + i])

		# Pop n mappings
		input_str = input_str[(mapping + 3):]

		# Get rid of fluff
		domain = domain[len('domain '):]
		co_domain = co_domain[len('codomain '):]

		domain = domain.split()
		co_domain = co_domain.split()

		function_true = True

		better_mapping = {}
		for line in mappings:
			line = line.split(' -> ')

			if line[0] not in better_mapping:
				better_mapping[line[0]] = line[1]
			else:
				function_true = False
				break


		if not function_true:
			print('not a function')
		else:
			injective_true = injective(domain, co_domain, better_mapping)
			surjective_true = surjective(domain, co_domain, better_mapping)

			if injective_true and not surjective_true:
				print('injective')
			elif not injective_true and surjective_true:
				print('surjective')
			elif not injective_true and not surjective_true:
				print('neither injective nor surjective')
			else:
				print('bijective')
	except:
		pass
