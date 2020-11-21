import math

c = input()

c, r = c.split(" ")

c = int(c)
r = int(r)

r -= c

c = math.pi * c * c
r = math.pi * r * r

print("%0.6f" %	((r) / (c) * 100))