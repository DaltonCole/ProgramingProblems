cases = int(input())

for i in range(cases):
	s = input()
	s = s.split(' ')
	s = s[1:]
	print(sub(s, 0, len(s)))

	subs = 0

	i0 = 0

	for j in range(1, len(s)):
		if s[j] > s[j-1]:
			i0 = j
			break

	while i0 != len(s):
		index = i0
		for j in range(i0, len(s)):
			if s[i0] >= s[j]:
				index += 1
			else:
				index -= 1
				break
		
