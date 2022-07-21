_ = input()

v = [0] + list(map(int, input().split()))


def good(i):
    for k in range(i, len(v), i):
        if v[k] <= v[k-i]:
            return False
    return True


for i in range(1, len(v) // 2):
    if good(i):
        print(i)
        quit()
print("ABORT!")
