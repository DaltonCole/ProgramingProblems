lower = 0
higher = 1001
guess = 500

while True:
	print(int(guess))

	response = input()

	if response == "correct":
		break
	elif response == "lower":
		higher = int(guess)
	else:
		lower = int(guess)

	guess = lower + ((higher - lower) // 2)