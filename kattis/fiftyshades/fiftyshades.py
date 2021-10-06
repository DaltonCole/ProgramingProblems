num = int(input())
count = 0

for _ in range(num):
    word = input().lower()
    if 'pink' in word or 'rose' in word:
        count += 1

if count > 0:
    print(count)
else:
    print('I must watch Star Wars with my daughter')
