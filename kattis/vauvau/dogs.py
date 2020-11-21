def dogs(count):
	if count == 0:
		print("none")
	elif count == 1:
		print("one")
	else:
		print("both")

def number(person):
	count = 0
	p_time = person % dog1_total
	if p_time <= a and p_time != 0:
		count += 1
	p_time = person % dog2_total
	if p_time <= c and p_time != 0:
		count += 1

	dogs(count)

a, b, c ,d = input().split()
a = int(a)
b = int(b)
c = int(c)
d = int(d)

p, m, g = input().split()
p = int(p)
m = int(m)
g = int(g)

dog1_total = a + b
dog2_total = c + d

number(p)
number(m)
number(g)

