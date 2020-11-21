message = input()

first_half = message[0:int(len(message) / 2)]
second_half = message[int(len(message) / 2):]

# First Half
sum_char = 0
for char in first_half:
	sum_char += (ord(char) - ord('A'))

sum_char %= 26

new_first_half = ''

for char in first_half:
	rotate = (sum_char + (ord(char) - ord('A'))) % 26
	new_first_half += chr(rotate + ord('A'))

# Second Half
sum_char = 0
for char in second_half:
	sum_char += (ord(char) - ord('A'))

sum_char %= 26

new_second_half = ''

for char in second_half:
	rotate = (sum_char + (ord(char) - ord('A'))) % 26
	new_second_half += chr(rotate + ord('A'))

# Merge
final_message = ''

for char1, char2 in zip(new_first_half, new_second_half):
	rotate = ((ord(char1) - ord('A')) + (ord(char2) - ord('A'))) % 26
	final_message += chr(rotate + ord('A'))

print(final_message)