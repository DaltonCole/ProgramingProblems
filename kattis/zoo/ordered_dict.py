import collections

num = 0
while True:
	num += 1
	cases = int(input())
	if cases == 0:
		break

	animal_dict = {}
	for i in range(cases):
		animal = input().split()[-1].lower()
		if animal not in animal_dict:
			animal_dict[animal] = 1
		else:
			animal_dict[animal] += 1

	ordered_animals = collections.OrderedDict(sorted(animal_dict.items()))

	print("List " + str(num) + ":")
	for key, value in ordered_animals.items():
		print(str(key) + " | " + str(value))