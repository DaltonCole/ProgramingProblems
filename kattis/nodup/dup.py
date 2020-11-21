line = input().split()

s = set()

for l in line:
	if l in s:
		print("no")
		quit()
	else:
		s.add(l)

print("yes")