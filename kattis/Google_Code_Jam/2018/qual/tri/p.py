mport random
n = 100000
t = random.sample(range(0, 1000000000), n)

print(1)
print(n)
for i in t:
	print(i, end=' ')
print()