from sys import stdin

GOAL = 'shiny gold'
bags = {}

# Get input
for line in stdin:
    root, tree = line.split(' bags contain ')
    # Add root bag to bags
    root = root.strip()
    if root not in bags:
        bags[root] = {}
    # If root bag cannot hold any other bags, go to next bag
    if 'no other bags' in tree:
        continue
    # Split tree bag into individual bags
    tree = tree.split('bag')
    for single_bag in tree:
        single_bag = single_bag.strip()
        single_bag = single_bag.split(' ')
        if len(single_bag) >= 3:
            bag_name = single_bag[-2] + ' ' + single_bag[-1]
            count = int(single_bag[-3])
            bags[root][bag_name] = count


def all_bags(current):
    if len(bags[current]) == 0:
        return 1

    count = 1
    for bag, num in bags[current].items():
        count += (num * all_bags(bag))
    return count

print(all_bags(GOAL) - 1)
