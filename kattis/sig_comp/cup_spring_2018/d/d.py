n, y = input().split()
n = int(n)
y = int(y)

found = set()

for i in range(y):
	found.add(int(input()))

for i in range(n):
	if i not in found:
		print(i)

print("Tek Jansen found %d data disks." % len(found))