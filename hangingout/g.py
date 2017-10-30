max_people, cases = input().split()
max_people = int(max_people)
cases = int(cases)

count = 0

for i in range(cases):
	direction, number= input().split()
	number = int(number)

	if direction == "enter":
		if max_people <= number:
			max_people += number
		else:
			count += 1
	else:
		max_people -= number

print(number)