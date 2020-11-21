import sys

number_of_key_words = int(input())

word = {}
word_count = {}

# Get all key words
for i in range(number_of_key_words):
	line = input().split()

	# Set key word count to 0
	word_count[line[0]] = 0

	# Get all words related to key wrod
	word[line[0]] = set()

	# Add related word
	for w in line[2:]:
		word[line[0]].add(w)


for line in sys.stdin:
	line = line.split()

	# For each word
	for w in line:
		for key, value in word.items():
			if w in value:
				word_count[key] += 1

max_num = max(word_count.values())

# Oh yea, lexicographical is a thing...
correct = []
for key, value in word_count.items():
	if value == max_num:
		correct.append(key)

correct = sorted(correct)

for i in correct:
	print(i)