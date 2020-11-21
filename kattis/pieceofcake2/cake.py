n, h, v = map(int, input().split())

a = h * v
b = (n-h) * v
c = h * (n-v)
d = (n-h) * (n-v)

e = max(a, b, c, d)

print((e * 4))
