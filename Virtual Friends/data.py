from random import randint

with open('lots', 'w') as f:
	f.write('10\n')

	for i in range(10):
		f.write('100000\n')
		for j in range(100000):
			f.write(str(randint(0,100000)) + ' ' + str(randint(0,100000)) + '\n')