lines, columns, classes = map(int, input().split())

# --- Get students --- #
students = {}

for _ in range(lines):
    row = input()
    for i, c in enumerate(row):
        if c not in students:
            students[c] = set()
        students[c].add(i)

ignore = []
for key in students.keys():
    if key in ignore:
        continue
    for key2 in students.keys():
        if key == key2:
            continue
        if key2 in ignore:
            continue

        if len(students[key].intersection(students[key2])) > 0:
            students[key] = students[key].union(students[key2])
            ignore.append(key2)

print(len(students.keys()) - len(ignore))
