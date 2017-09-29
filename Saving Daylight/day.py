import sys

for line in sys.stdin:
	month, day, year, time_start, time_end = line.split()

	hour_start, minute_start = time_start.split(':')
	hour_end, minute_end = time_end.split(':')

	hour_start = int(hour_start)
	hour_end = int(hour_end)
	minute_start = int(minute_start)
	minute_end = int(minute_end)

	hour = hour_end - hour_start
	minute = minute_end - minute_start

	if minute < 0:
		hour -= 1
		minute += 60

	print("%s %s %s %d hours %d minutes" % (month, day, year, hour, minute))