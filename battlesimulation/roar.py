from itertools import permutations

line = input()

perms = [''.join(p) for p in permutations('RBL')]

print(perms)