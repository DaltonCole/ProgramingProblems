e, f, c = input().split()

e = int(e)
f = int(f)
c = int(c)

total = 0

empty_bottles = e + f

while empty_bottles >= c:
	total += empty_bottles // c
	temp = empty_bottles
	empty_bottles = empty_bottles // c
	empty_bottles += temp % c

print(total)
