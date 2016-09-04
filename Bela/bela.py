def points(number, suit, dominate_suit):
	if suit is dominate_suit:
		if number is "A":
			return 11
		elif number is "K":
			return 4
		elif number is "Q":
			return 3
		elif number is "J":
			return 20
		elif number is "T":
			return 10
		elif number is "9":
			return 14
		elif number is "8":
			return 0
		elif number is "7":
			return 0
	else:
		if number is "A":
			return 11
		elif number is "K":
			return 4
		elif number is "Q":
			return 3
		elif number is "J":
			return 2
		elif number is "T":
			return 10
		elif number is "9":
			return 0
		elif number is "8":
			return 0
		elif number is "7":
			return 0

rounds, dominate_suit = input().split(" ")

total = 0

for i in range(int(rounds) * 4):
	hand = input()
	total += points(hand[0], hand[1], dominate_suit)

print(total) 