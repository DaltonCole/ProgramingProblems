s = input()

a = True
b = False
c = False

for i in s:
	if i == "A":
		a,b = b,a
	elif i == "B":
		b,c = c,b
	elif i == "C":
		a,c = c,a

if a == True:
	print("1")
if b == True:
	print("2")
if c == True:
	print("3")