time = input()

print(time)

junk, time = time.split(" ")

time = int(time)

tasks = input()

tasks = tasks.split(" ")

print(tasks)

for i in range(len(tasks)):
	time -= int(tasks[i])
	if time < 0:
		print(i)
		break