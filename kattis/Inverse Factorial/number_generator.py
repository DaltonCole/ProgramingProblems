base = 1
current = 1
while(len(str(current)) < 50000):
	base += 1
	current *= base
	#if(len(str(current)) % 1000 == 0):
	#	print(current)

with open('really_big', 'w') as f:
	f.write(str(current))
	f.write('\n\n\n')
	f.write(str(base))