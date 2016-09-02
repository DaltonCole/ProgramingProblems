inputs = int(input())

tree_list = input()

tree_list = tree_list.split(" ")

tree_list = [int(i) for i in tree_list]

m = 0

tree_list.sort(reverse=True)

for i in range(len(tree_list)):
	if i + tree_list[i] + 2 > m:
		m = i + tree_list[i] + 2

print(m)