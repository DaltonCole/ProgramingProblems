a, b, c = input().split(' ')
a = int(a)
b = int(b)
c = int(c)

arrival1, depart1 = input().split(' ')
arrival1 = int(arrival1)
depart1 = int(depart1)

arrival2, depart2 = input().split(' ')
arrival2 = int(arrival2)
depart2 = int(depart2)

arrival3, depart3 = input().split(' ')
arrival3 = int(arrival3)
depart3 = int(depart3)

all_list = []

for i in range(arrival1, depart1):
	all_list.append(i)

for i in range(arrival2, depart2):
	all_list.append(i)

for i in range(arrival3, depart3):
	all_list.append(i)

all_list.sort()

current_count = 0
total = 0

for i in range(0, 101):
	current_count = all_list.count(i)

	if current_count == 1:
		total += a
	elif current_count == 2:
		total += b * 2
	elif current_count == 3:
		total += c * 3

print(total)