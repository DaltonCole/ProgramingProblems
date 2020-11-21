cases = int(input())

for case in range(cases):
    line = input().split()
    line = list(map(int, line))
    
    if len(line) == 3:
        print('1')
        continue
    
    for i in range(1, len(line) - 2):
        if line[i] + 1 == line[i+1]:
        	pass
        else:
        	if i + 2 < len(line) and line[i] + 1 == line[i+2]:
	        	print(i + 1)
	        else:
	        	print(i)
        	break