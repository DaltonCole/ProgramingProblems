cost = float(input())
lawns = int(input())

total_cost = 0

for i in range(lawns):
	user_input = input()
	w, l = user_input.split(' ')
	w = float(w)
	l = float(l)

	total_cost += cost * w * l

print(str.format('{0:.7f}', total_cost))