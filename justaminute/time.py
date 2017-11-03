cases = int(input())

total_minutes = 0
total_seconds = 0

for i in range(cases):
	displayed, seconds = input().split()
	total_minutes += int(displayed)
	total_seconds += int(seconds)


sl_time = total_seconds / (60 * total_minutes)

if sl_time <= 1:
	print("measurement error")
else:
	print(sl_time)