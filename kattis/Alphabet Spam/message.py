text = input()

white = 0
lower = 0
upper = 0
symbols = 0

lower_array = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']
upper_array = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z']


for i in text:
	if i in lower_array:
		lower += 1
	elif i in upper_array:
		upper += 1
	elif i == "_":
		white += 1
	else:
		symbols += 1

print("%0.16f" % (white / len(text)))
print("%0.16f" % (lower / len(text)))
print("%0.16f" % (upper / len(text)))
print("%0.16f" % (symbols / len(text)))