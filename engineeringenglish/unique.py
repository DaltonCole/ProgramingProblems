import sys

word_list = set()

for line in sys.stdin:
	for word in line.split():
		if word.lower() not in word_list:
			print(word, end=' ')
			word_list.add(word.lower())
		else:
			print('. ', end='')
	print()
