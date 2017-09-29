import sys

cases = int(input())

lines = []
for line in sys.stdin:
	lines.append(line)


sounds = []
other = []

for line in lines:
	if "?" in line:
		temp = []
		sound_string = ''
		for s in sounds:
			if s not in other:
				temp.append(s)

		for s in temp:
			if s not in other:
				sound_string += s + ' '

		sounds = []
		other = []
		print(sound_string[0:-1])
		continue

	if 'goes' in line:
		other.append(line.split()[2])
	else:
		sounds = line.split()