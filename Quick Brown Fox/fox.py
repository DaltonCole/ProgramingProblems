cases = int(input())

alphabet = []

full_alphabet = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']

for i in range(cases):
	text = input()
	text = text.lower()

	if "a" in text:
		alphabet.append("a")
	if "b" in text:
		alphabet.append("b")
	if "c" in text:
		alphabet.append("c")
	if "d" in text:
		alphabet.append("d")
	if "e" in text:
		alphabet.append("e")
	if "f" in text:
		alphabet.append("f")
	if "g" in text:
		alphabet.append("g")
	if "h" in text:
		alphabet.append("h")
	if "i" in text:
		alphabet.append("i")
	if "j" in text:
		alphabet.append("j")
	if "k" in text:
		alphabet.append("k")
	if "l" in text:
		alphabet.append("l")
	if "m" in text:
		alphabet.append("m")
	if "n" in text:
		alphabet.append("n")
	if "o" in text:
		alphabet.append("o")
	if "p" in text:
		alphabet.append("p")
	if "q" in text:
		alphabet.append("q")
	if "r" in text:
		alphabet.append("r")
	if "s" in text:
		alphabet.append("s")
	if "t" in text:
		alphabet.append("t")
	if "u" in text:
		alphabet.append("u")
	if "v" in text:
		alphabet.append("v")
	if "w" in text:
		alphabet.append("w")
	if "x" in text:
		alphabet.append("x")
	if "y" in text:
		alphabet.append("y")
	if "z" in text:
		alphabet.append("z")

	if len(alphabet) == 26:
		print("pangram")
	else:
		missing = []
		for i in full_alphabet:
			if i not in alphabet:
				missing.append(i)
		print("missing " + ''.join(missing))
	alphabet = []
	