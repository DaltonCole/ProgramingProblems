operators = [' + ', ' - ', ' * ', ' // ']
def function(a):
	for op1 in operators:
		for op2 in operators:
			for op3 in operators:
				b = int(eval("4{}4{}4{}4".format(op1, op2, op3)))
				if a == b:
					return "4{}4{}4{}4".format(op1, op2, op3)
	return False


cases = int(input())

for i in range(cases):
	answer = int(input())

	evaluated = function(answer)

	if evaluated == False:
		print("no solution")
	else:
		evaluated = evaluated.replace(' // ', ' / ')
		print("{} = {}".format(evaluated, answer))