from sys import stdin

GOAL = 'shiny gold'
bags = {}

# Get input
for line in stdin:
    root, tree = line.split(' bags contain ')
    # Add root bag to bags
    root = root.strip()
    if root not in bags:
        bags[root] = set()
    # If root bag cannot hold any other bags, go to next bag
    if 'no other bags' in tree:
        continue
    # Split tree bag into individual bags
    tree = tree.split('bag')
    for single_bag in tree:
        single_bag = single_bag.strip()
        single_bag = single_bag.split(' ')
        if len(single_bag) >= 2:
            single_bag = single_bag[-2] + ' ' + single_bag[-1]
            bags[root].add(single_bag)

for key, value in bags.items():
    print(f'{key}: {value}')


reverse_bags = {key: set() for key in bags.keys()}
for key, value in bags.items():
    for bag in value:
        reverse_bags[bag].add(key)

print('\nReverse Bag:')
for key, value in reverse_bags.items():
    print(f'{key}: {value}')


print()
def add_parents(bags, root, current):
    bag_list = list(bags[current])
    for bag in bag_list:
        if bag not in bags[root]:
            print(bag)
            bags[root].add(bag)
            add_parents(bags, root, bag)

bag_list = list(reverse_bags[GOAL])
for bag in bag_list:
    print(bag)
    add_parents(reverse_bags, GOAL, bag)

print()
for key, value in reverse_bags.items():
    print(f'{key}: {value}')

print(len(reverse_bags[GOAL]))
