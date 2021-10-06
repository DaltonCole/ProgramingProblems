def harshadnumbers(num: str):
    div = 0
    for c in num:
        div += int(c)

    return int(num) % div == 0

num = input()

while True:
    if harshadnumbers(num):
        print(num)
        break
    num = str(int(num) + 1)
