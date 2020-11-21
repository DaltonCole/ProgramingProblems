kitten = int(input())

tree = {}

while True:
	branch = input().split()

	if branch[0] == '-1':
		break

	branch = list(map(int, branch))

	for i in range(1, len(branch)):
		tree[branch[i]] = branch[0]

print(kitten, end='')

while kitten in tree:
	kitten = tree[kitten]
	print(' ' + str(kitten), end='')

print()