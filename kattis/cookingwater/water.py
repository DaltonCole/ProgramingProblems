number_of_lines = int(input())

similar_elements = set()

for i in range(number_of_lines):
	start, end = list(map(int, input().split()))

	if i == 0:
		similar_elements = set(list(range(start, end + 1)))

	if len(similar_elements) == 0:
		continue

	similar_elements = similar_elements.intersection(set(list(range(start, end + 1))))

if len(similar_elements) != 0:
	print('gunilla has a point')
else:
	print('edward is right')