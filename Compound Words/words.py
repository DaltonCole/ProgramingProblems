import sys

words = []

for line in sys.stdin:
	words += line.split()

s = set()

for i in range(len(words)):
	for j in range(len(words)):
		if i == j:
			continue
		else:
			s.add(words[i] + words[j])


s = sorted(s)

for word in s:
	print(word)