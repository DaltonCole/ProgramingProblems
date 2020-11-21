victory = [('Province', 8), ('Duchy', 5), ('Estate', 2)]
treasure = [('Gold', 6), ('Silver', 3), ('Copper', 0)]

g, s, c = list(map(int, input().split()))

g *= 3
s *= 2

total = g + s + c

choosen_victory = ''
choosen_treasure = ''

for v in victory:
	if v[1] <= total:
		choosen_victory = v[0]
		break

for t in treasure:
	if t[1] <= total:
		choosen_treasure = t[0]
		break

if choosen_victory != '':
	print('{} or '.format(choosen_victory), end='')

print(choosen_treasure)