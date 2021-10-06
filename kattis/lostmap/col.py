cities = int(input())

dist = [[] for _ in range(cities)]
for i in range(cities):
    line = map(int, input().split())
    for j, c in enumerate(line):
        if c == 0:
            continue
        dist[j].append((i+1, j+1, c))

for l in dist:
    print(l)
