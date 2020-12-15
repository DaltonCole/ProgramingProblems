time = int(input())

sched = input().split(',')
sched = sorted(list(map(int, filter(lambda x: x != 'x', sched))))
print(sched)

close = [0] * len(sched)

for i, num in enumerate(sched):
    close[i] = int(time / num) * num
    if close[i] < time:
        close[i] += num

close = list(zip(close, sched))
close = sorted(close, key=lambda x: x[0])

print((close[0][0] - time) * close[0][1])

