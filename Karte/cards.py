cards = input()

# Card index
i = 0

# True if user inputs the same card twice
wrong = False

# Fill array with cards
p = []
k = []
h = []
t = []

# Go through input string
while i < len(cards):
	# If card suit is X
	if cards[i] == "P":
		# Add the two digit number to the array
		if cards[i+1:i+3] not in p:
			p.append(cards[i+1:i+3])
		else:
			# If not unique, mark as wrong and print the wrong key
			wrong = True
			print("GRESKA")
			break
	elif cards[i] == "K":
		if cards[i+1:i+3] not in k:
			k.append(cards[i+1:i+3])
		else:
			wrong = True
			print("GRESKA")
			break
	elif cards[i] == "H":
		if cards[i+1:i+3] not in h:
			h.append(cards[i+1:i+3])
		else:
			wrong = True
			print("GRESKA")
			break
	elif cards[i] == "T":
		if cards[i+1:i+3] not in t:
			t.append(cards[i+1:i+3])
		else:
			wrong = True
			print("GRESKA")
			break
	# Increment index by three; one for suit, two for number
	i += 3

# If not wrong, print answer
if wrong == False:
	print(str(13 - len(p)) + " " + str(13 - len(k)) + " " + str(13 - len(h)) + " " + str(13 - len(t)))