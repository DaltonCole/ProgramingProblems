_ = input()

broken = list(map(int, input().split()))

reserved = list(map(int, input().split()))

actually_broken = []
actually_reserved = []

for i in broken:
	if i not in reserved:
		actually_broken.append(i)

for i in reserved:
	if i not in broken:
		actually_reserved.append(i)

broken = actually_broken
reserved = actually_reserved

other = []

for i in broken:
	if i - 1 in reserved:
		reserved.remove(i-1)
	elif i + 1 in reserved:
		reserved.remove(i+1)
	else:
		other.append(i)

print(len(other))