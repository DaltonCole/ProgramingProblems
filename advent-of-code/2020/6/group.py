from sys import stdin

group = set()
count = 0

for line in stdin:
    line = line.rstrip()
    if line == '':
        count += len(group)
        group = set()
    else:
        for c in line:
            group.add(c)

count += len(group)
print(count)
