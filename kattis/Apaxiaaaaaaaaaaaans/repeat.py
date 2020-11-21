original = input()

new = original[0]

for i in original:
	if i != new[-1]:
		new += i

print(new)