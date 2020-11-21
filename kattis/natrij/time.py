hour_a, minute_a, sec_a = input().split(':')
hour_a = int(hour_a)
minute_a = int(minute_a)
sec_a = int(sec_a)

hour_b, minute_b, sec_b = input().split(':')
hour_b = int(hour_b)
minute_b = int(minute_b)
sec_b = int(sec_b)

new_sec = (sec_b - sec_a)

if new_sec < 0:
	new_sec %= 60
	minute_a += 1

new_minute = (minute_b - minute_a)

if new_minute < 0:
	new_minute %= 60
	hour_a += 1

new_hour = (hour_b - hour_a) % 24

if hour_a == hour_b and minute_a == minute_b and sec_a == sec_b:
	print("24:00:00")
else:
	print("{:02d}:{:02d}:{:02d}".format(new_hour, new_minute, new_sec))