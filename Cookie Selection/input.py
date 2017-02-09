from random import randint

with open('test', 'w') as f:
	numbers = 0
	octo = 0

	for i in range(0, 600000):
		a = randint(1, 4)

		if(a == 1 and octo < numbers):
			octo += 1
			f.write('#\n')
		else:
			numbers += 1
			f.write(str(randint(1, 300000000))+ '\n')
