cases = int(input())

for i in range(cases):
	input()
	groups = int(input())
	candy = 0

	for i in range(groups):
		candy += int(input())

	if candy % groups == 0:
		print("YES")
	else:
		print("NO")