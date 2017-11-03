def men_or_woman(char):
	if char == 'W':
		return -1
	return 1

max_difference = int(input())

line = list(input())

# Positive is men, negative is women
current_difference = 0

count = 0

for i in range(len(line)):
	gender = men_or_woman(line[i])
	if abs(current_difference + gender) <= max_difference:
		current_difference += gender
		count += 1
	elif i < len(line) - 1: # Not last person in line
		other_gender = men_or_woman(line[i+1])
		if other_gender == gender:
			break
		elif abs(current_difference + other_gender) <= max_difference:
			count += 1
			line[i], line[i+1] = line[i+1], line[i]
			current_difference += other_gender

print(count)