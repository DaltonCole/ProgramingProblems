from math import ceil

apples, sack = input().split()
apples = int(apples)
sack = int(sack)

distance = 1

while apples > sack:
	apples -= ceil(apples / sack)
	distance += 1

distance += apples

print(distance)