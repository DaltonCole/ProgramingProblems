from sys import stdin

def intersect(group) -> int:
    return len(group[0].intersection(*group))

group = []
count = 0

for line in stdin:
    line = line.rstrip()
    if line == '':
        count += intersect(group)
        group = []
    else:
        group.append(set())
        for c in line:
            group[-1].add(c)

count += intersect(group)
print(count)
