import datetime

user_input = input()

day, month = user_input.split(' ')

day = int(day)
month = int(month)

date = datetime.datetime(2009, month, day)

day_of_week = date.weekday()

if day_of_week == 0:
	print("Monday")
elif day_of_week == 1:
	print("Tuesday")
elif day_of_week == 2:
	print("Wednesday")
elif day_of_week == 3:
	print("Thursday")
elif day_of_week == 4:
	print("Friday")
elif day_of_week == 5:
	print("Saturday")
elif day_of_week == 6:
	print("Sunday")