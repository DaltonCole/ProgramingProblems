lines = int(input())

d = {}

for i in range(lines):
	school, team = input().split()

	if school not in d and len(d) < 12:
		print("{} {}".format(school, team))
		d[school] = team