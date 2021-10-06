from sys import stdin

total = 0


# https://brilliant.org/wiki/shunting-yard-algorithm/#
def shunting_yard_algorithm(math):
    queue = []
    stack = []

    while len(math) > 0:
        token = math.pop(0)
        if token.isdigit():
            queue.append(int(token))
        elif token in ['*', '+']:
            if token == '*':
                while len(stack) > 0 and stack[-1] == '+':
                    queue.append(stack.pop())
            stack.append(token)
        elif token == '(':
            stack.append(token)
        elif token == ')':
            while stack[-1] != '(':
                queue.append(stack.pop())
            stack.pop()
    while len(stack) > 0:
        queue.append(stack.pop())

    stack = []
    for op in queue:
        if type(op) is int:
            stack.append(op)
        else:
            if op == '+':
                stack.append(stack.pop() + stack.pop())
            else:
                stack.append(stack.pop() * stack.pop())

    return stack[0]


for line in stdin:
    line = ''.join(line.split())
    line = [x for x in line]
    total += shunting_yard_algorithm(line)

print(total)
