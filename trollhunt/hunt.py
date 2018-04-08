from math import ceil

bridges, knights, min_knights = list(map(int, input().split()))

bridges -= 1

a = knights // min_knights
b = int(ceil(bridges / a))

print(b)