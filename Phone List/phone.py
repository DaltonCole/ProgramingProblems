import datetime

test_cases = int(input())

for i in range(test_cases):
	inputs = int(input())

	l = []

	leave = False

	for j in range(inputs):
		number = input()

		if leave == False:
			for k in l:
				if number.startswith(str(k)):
					leave = True
					break
		else:
			break

		l.append(number)

	if leave == True:
		print("NO")
	else:
		print("YES")