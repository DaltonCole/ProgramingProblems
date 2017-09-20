import math

cloud_count = 0

while True:
	cloud_count += 1

	max_width, words = input().split()

	if max_width == "0" and words == "0":
		quit()

	max_width = int(max_width)
	words = int(words)
	max_occurence = 0

	# [['word', int_occurences, int_height, int_width], ...]
	word_list = []

	for line in range(words):
		word, occurence = input().split()
		occurence = int(occurence)
		word_list.append([word, occurence])
		if occurence > max_occurence:
			max_occurence = occurence

	word_list.sort(key=lambda x: x[0])

	for word in word_list:
		# P=8+⌈40⋅(cw−4) / (cmax−4)⌉
		font_size = 8 + math.ceil(40 * (word[1] - 4) / (max_occurence - 4))
		word.append(font_size)
		# ⌈9/16⋅t⋅P⌉
		font_width = math.ceil((9/16) * len(word[0]) * font_size)
		word.append(font_width)

	current_width = 0
	current_row_height = 0
	total_height = 0

	for word in word_list:
		if current_width + word[3] > max_width:
			total_height += current_row_height
			current_row_height = 0
			current_width = 0
		current_width += word[3] + 10
		if current_row_height < word[2]:
			current_row_height = word[2]

	total_height += current_row_height

	print("CLOUD " + str(cloud_count) + ": " + str(total_height))