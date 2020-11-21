import sys

for line in sys.stdin:
    line = line.split()

    numbers = []
    name = ''

    for i in line:
    	try:
    		numbers.append(float(i))
    	except:
    		name += ' ' + i
    
    average = 0
    for i in numbers:
    	average += i
    average /= len(numbers)

    print(str(average) + name)