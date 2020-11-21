import math

# 7
def t7(number):
	return number

def t6(number):
	return math.log2(number) * number

def t5(number):
	return pow(number, 2)

def t4(number):
	return pow(number, 3)

def t3(number):
	return pow(number, 4)

def t2(number):
	base = 1 << number
	return base

def t1(number, m): # Slow one
	answer = 1
	for i in range(number, 1, -1):
		answer = i * answer
		if answer > m:
			print("TLE")
			quit()
	print("AC")
	quit()
	#return math.factorial(number)


m, n, t = input().split()

m = int(m)
n = int(n)
t = int(t)

total = 0

if t == 1:
	total = t1(n, m)
elif t == 2:
	total = t2(n)
elif t == 3:
	total = t3(n)
elif t == 4:
	total = t4(n)
elif t == 5:
	total = t5(n)
elif t == 6:
	total = t6(n)
elif t == 7:
	total = t7(n)


if total <= m:
	print("AC")
else:
	print("TLE")