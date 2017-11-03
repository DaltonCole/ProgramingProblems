d = {}
d['1'] = 0
d['2'] = 0
d['3'] = 0
d['4'] = 0
d['5'] = 0
d['6'] = 0

_ = input()

dice = input().split()

num = {}
for enum, i in enumerate(dice):
	d[i] += 1
	num[i] = enum

max_v = '0'

for key, value in d.items():
	if value == 1 and key > max_v:
		max_v = key

if max_v == '0':
	print("none")
else:
	print(num[max_v] + 1)