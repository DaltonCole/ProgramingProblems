from sys import stdin

total = 0


def calc(math):
    num = None
    op = None
    while len(math) > 0:
        if math[0].isdigit():
            tmp = int(math.pop(0))
            if num is None:
                num = tmp
            else:
                if op == '*':
                    num *= tmp
                else:
                    num += tmp
        elif math[0] == '(':
            math.pop(0)
            tmp = calc(math)
            if num is None:
                num = tmp
            else:
                if op == '*':
                    num *= tmp
                else:
                    num += tmp
        elif math[0] == ')':
            math.pop(0)
            return num
        else:
            op = math[0]
            math.pop(0)
    return num


for line in stdin:
    line = ''.join(line.split())
    line = [x for x in line]
    total += calc(line)

print(total)
