, cost = input().split()
cost = int(cost)

# List of ints of times
times = list(map(int, input().split()))

# List of times - cost
off_set_times = []
for num in times:
	off_set_times.append(num - cost)

max_so_far = 0
max_ending_here = 0

for i in off_set_times:
	max_ending_here += i
	if max_ending_here < 0:
		max_ending_here = 0
	if max_so_far < max_ending_here:
		max_so_far = max_ending_here

print(max_so_far)


"""

18 35 6 80 15 21

-2 15 -14 60 -5 1

-2 13 -1 59 54 55
		 MAX


21 17 35 6 80 15 21

1 -2 15 -14 60 -5 1

1 -2 13 -1 59 54 55

best = max(best, current + current[-1])
current + current[-1]

21 21 21 21 0 40
"""