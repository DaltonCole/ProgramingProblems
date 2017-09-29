from math import ceil

b, br, bs, A, As = input().split()

b = float(b)
br = float(br)
bs = float(bs)
A = float(A)
As = float (As)

Ar = ((br - b) * bs / As) + A

if Ar == ceil(Ar):
	Ar += 1

print(int(ceil(Ar)))