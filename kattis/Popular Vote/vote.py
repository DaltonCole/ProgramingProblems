cases = int(input())

for i in range(cases):
	votes = int(input())
	vote_list = []

	for j in range(votes):
		vote_list.append(int(input()))

	max_value = -1
	index = -1
	total_value = 0

	for v in vote_list:
		if max_value < v:
			max_value = v
			index = vote_list.index(v)
		total_value += v

	same_votes = 0

	for v in vote_list:
		if v == max_value:
			same_votes += 1

	if same_votes > 1:
		print("no winner")
	elif (max_value > total_value - max_value):
		print("majority winner " + str(index + 1))
	else:
		print("minority winner " + str(index + 1))