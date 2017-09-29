

while True:
	length = int(input())

	if length == 0:
		break

	first_list = []
	first_sorted_list = []
	second_list = []
	second_sorted_list = []

	for i in range(length):
		temp = int(input())
		first_list.append(temp)
		first_sorted_list.append(temp)
	for i in range(length):
		second_sorted_list.append(int(input()))

	first_sorted_list.sort()
	second_sorted_list.sort()

	for a in first_list:
		index = first_sorted_list.index(a)
		second_list.append(second_sorted_list[index])

	for i in second_list:
		print(i)

	print()

