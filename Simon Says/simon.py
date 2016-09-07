cases = int(input())

for i in range(cases):
	text = input()

	if text.startswith("Simon says "):
		print(text[11:(len(text))])