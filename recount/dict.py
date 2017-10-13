
votes = {}

while True:
	i = input()

	if i == '***':
		break

	if i not in votes:
		votes[i] = 1
	else:
		votes[i] += 1

person = ''
count = 0

for key, value in votes.items():
	if value > count:
		person = key
		count = value

runoff = False

for key, value in votes.items():
	if key == person:
		continue
	elif value == count:
		runoff = True
		break

if runoff == True:
	print("Runoff!")
else:
	print(person)